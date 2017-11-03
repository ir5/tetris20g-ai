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
        self.per = np.int64(((self.dim + 7) // 8) * 8)

        self.data = bitarray()
        for filename in filenames:
            f = open(filename, 'rb')
            s = bitarray(endian='little')
            s.fromfile(f)
            assert len(s) % (2 * self.per) == 0
            self.data.extend(s)

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
            self.l = L.Linear(dim, 1, nobias=True)

    def __call__(self, x):
        return self.l(x)

    def dump(self):
        return ' '.join(np.char.mod('%.14f', self.l.W.data[0]))


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

        self.accuracy = sum(diff.data > 0)[0] / diff.shape[0]
        reporter.report({'accuracy': self.accuracy}, self)

        return self.loss


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--batchsize', type=int, default=128)
    parser.add_argument('--epoch', type=int, default=20)
    parser.add_argument('--gpu', type=int, default=-1)
    parser.add_argument('--loaderjob', '-j', type=int, default=1)
    parser.add_argument('--dim', type=int, default=8184)
    parser.add_argument('--lr', type=float, default=1e-3)
    parser.add_argument('--weight-decay', type=float, default=0)
    parser.add_argument('--train', type=str, nargs='+', default=None)
    parser.add_argument('--val', type=str, nargs='+', default=None)
    parser.add_argument('--dump-from', type=str, default=None)
    args = parser.parse_args()

    model = RankLoss(LinearRegression(args.dim))
    if args.gpu >= 0:
        chainer.cuda.get_device_from_id(args.gpu).use()
        model.to_gpu()

    if args.dump_from is not None:
        chainer.serializers.load_npz(args.dump_from, model.predictor)
        print(model.predictor.dump())
        return

    optimizer = chainer.optimizers.Adam(alpha=args.lr)
    optimizer.setup(model)
    optimizer.add_hook(chainer.optimizer.WeightDecay(args.weight_decay))

    train = PreprocessedDataset(args.train, args.dim)
    val = PreprocessedDataset(args.val, args.dim)
    print('dataset load: done')

    train_iter = chainer.iterators.MultiprocessIterator(
        train, args.batchsize, n_processes=args.loaderjob)
    val_iter = chainer.iterators.MultiprocessIterator(
        val, args.batchsize, repeat=False, n_processes=args.loaderjob)

    updater = training.StandardUpdater(train_iter, optimizer, device=args.gpu)
    trainer = training.Trainer(updater, (args.epoch, 'epoch'),
                               out='results')
    trainer.extend(extensions.Evaluator(val_iter, model, device=args.gpu))
    trainer.extend(extensions.LogReport())
    trainer.extend(extensions.snapshot_object(
        model.predictor, filename='model_{.updater.epoch}'))
    trainer.extend(extensions.PrintReport(
        ['epoch', 'main/loss', 'validation/main/loss',
         'main/accuracy', 'validation/main/accuracy',
         'elapsed_time']))
    trainer.extend(extensions.ProgressBar())

    trainer.run()


if __name__ == '__main__':
    main()
