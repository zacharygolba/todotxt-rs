#![feature(test)]

extern crate test;


use test::Bencher;
use todotxt::prelude::*;

static A: &str = include_str!("../../fixtures/todo.txt");
static B: &str = "x 2011-03-02 2011-03-01 Review Tim's pull request +TodoTxtTouch @github";

#[bench]
fn list(bench: &mut Bencher) {
    bench.iter(|| {
        assert!(A.tasks().count() > 0);
    });
}

#[bench]
fn task(bench: &mut Bencher) {
    bench.iter(|| {
        assert!(B.tasks().count() > 0);
    });
}

#[bench]
#[cfg(feature = "rayon")]
fn par_list(bench: &mut Bencher) {
    bench.iter(|| {
        assert!(A.par_tasks().count() > 0);
    });
}

#[bench]
#[cfg(feature = "rayon")]
fn par_task(bench: &mut Bencher) {
    bench.iter(|| {
        assert!(B.par_tasks().count() > 0);
    });
}
