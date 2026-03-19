use tokio::sync::mpsc;

pub fn create_test_channel<T>() -> (mpsc::Sender<T>, mpsc::Receiver<T>) {
    tokio::sync::mpsc::channel::<T>(4)
}
