import argparse
from bitarray import bitarray
import chainer
import chainer.functions as F
import chainer.links as L
from chainer import reporter
from chainer import training
from chainer.training import extensions
import numpy as np


class PreprocessedDataset(chainer.dataset.DatasetMixin):

    def __init__(self, filenames, dim):
        self.dim = dim
        self.per = ((self.dim + 7) // 8) * 8

        self.data = bitarray()
        for filename in filenames:
            f = open(filename, 'rb')
            s = bitarray(f.read())
            self.data.extend(s)
        print(len(self.data))

    def __len__(self):
        return len(self.data) // (2 * self.per)

    def get_example(self, i):
        return self._get(2 * i), self._get(2 * i + 1)

    def _get(self, i):
        return np.array(self.data[self.per*i:self.per*(i+1)],
                        dtype=np.float32)


class LinearRegression(chainer.Chain):

    def __init__(self, dim):
        super(LinearRegression, self).__init__()
        self.dim = dim
        with self.init_scope():
            self.l = L.Linear(dim, 1)

    def __call__(self, x):
        return self.l(x)


class RankLoss(chainer.Chain):

    def __init__(self, predictor):
        super(RankLoss, self).__init__()

        with self.init_scope():
            self.predictor = predictor

    def __call__(self, x_high, x_low):
        y_high = self.predictor(x_high)
        y_low = self.predictor(x_low)
        diff = y_high - y_low
        self.loss = F.average(F.log(1 + F.exp(-diff)))
        reporter.report({'loss': self.loss}, self)

        return self.loss


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--batchsize', type=int, default=128)
    parser.add_argument('--epoch', type=int, default=20)
    parser.add_argument('--gpu', type=int, default=0)
    parser.add_argument('--loaderjob', '-j', type=int, default=1)
    parser.add_argument('--dim', type=int, default=8184)
    parser.add_argument('--train', type=str, nargs='+', default=None)
    parser.add_argument('--test', type=str, nargs='+', default=None)
    args = parser.parse_args()

    model = RankLoss(LinearRegression(args.dim))
    if args.gpu >= 0:
        chainer.cuda.get_device_from_id(args.gpu).use()
        model.to_gpu()
    print('model: done')

    optimizer = chainer.optimizers.Adam()
    optimizer.setup(model)

    train = PreprocessedDataset(args.train, args.dim)
    print('load train: done')
    test = PreprocessedDataset(args.test, args.dim)
    print('load test: done')

    """
    train_iter = chainer.iterators.SerialIterator(train, args.batchsize)
    test_iter = chainer.iterators.SerialIterator(test, args.batchsize,
                                                 repeat=False, shuffle=False)
    """
    train_iter = chainer.iterators.MultiprocessIterator(
        train, args.batchsize, n_processes=args.loaderjob)
    test_iter = chainer.iterators.MultiprocessIterator(
        test, args.batchsize, repeat=False, n_processes=args.loaderjob)

    updater = training.StandardUpdater(train_iter, optimizer, device=args.gpu)
    trainer = training.Trainer(updater, (args.epoch, 'epoch'))
    trainer.extend(extensions.Evaluator(test_iter, model, device=args.gpu))
    trainer.extend(extensions.LogReport())
    trainer.extend(extensions.PrintReport(
        ['epoch', 'main/loss', 'validation/main/loss', 'elapsed_time']))
    trainer.extend(extensions.ProgressBar())

    trainer.run()


if __name__ == '__main__':
    main()
