use std::fmt::Error;
use std::sync::mpsc::{Receiver, Sender};

#[warn(dead_code)]
const MIN_WORKER: i32 = 1;
#[warn(dead_code)]
const DEFAULT_WORKERS: i32 = 16;


type GenerateFunc<T> = fn(source: Receiver<T>);
type MapFunc<T> = fn(item: T, writer: Writer);
type VoidMapFunc<T> = fn(item: T);
type MapperFunc<T> = fn(item: T, writer: Writer, cancel: fn(Error));
type ReducerFunc<T> = fn(pipe: Sender<T>, writer: Writer, cancel: fn(Error));
type VoidReducerFunc<T> = fn(pipe: Sender<T>, cancel: fn(Error));
type Option = fn(opts: &mut MapReduceOption);

trait Writer {
    fn write<T>(&self, v: T);
}

struct MapReduceOption {
    worker: i32
}
