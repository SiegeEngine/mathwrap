#![macro_use]

macro_rules! impl_op {
    ($Op:ident, $op:ident, $T:ty, $Tx:expr) => {
        impl<F: FullFloat> $Op<$T> for $T {
            type Output = $T;
            fn $op(self, rhs: $T) -> $T {
                $Tx((self.0).$op(rhs.0))
            }
        }
    };
}

macro_rules! impl_op_f {
    ($Op:ident, $op:ident, $T:ty, $Tx:expr) => {
        impl<F: FullFloat> $Op<F> for $T {
            type Output = $T;
            fn $op(self, rhs: F) -> $T {
                $Tx((self.0).$op(rhs))
            }
        }
    };
}

macro_rules! impl_uop {
    ($Op:ident, $op:ident, $T:ty, $Tx:expr) => {
        impl<F: FullFloat> $Op for $T {
            type Output = $T;
            fn $op(self) -> $T {
                $Tx((self.0).$op())
            }
        }
    };
}

macro_rules! impl_aop {
    ($Op:ident, $op:ident, $T:ty, $Tx:expr) => {
        impl<F: FullFloat> $Op for $T {
            fn $op(&mut self, rhs: $T) {
                (self.0).$op(rhs.0)
            }
        }
    };
}

macro_rules! impl_aop_f {
    ($Op:ident, $op:ident, $T:ty, $Tx:expr) => {
        impl<F: FullFloat> $Op<F> for $T {
            fn $op(&mut self, rhs: F) {
                (self.0).$op(rhs)
            }
        }
    };
}
