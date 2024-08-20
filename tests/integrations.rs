use blur::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[test]
fn small_radius() {
    let img = image::open("assets/bg.jpg").unwrap().to_rgb8();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let img_blurred = blur(3, 10.0, img);
        img_blurred.save("assets/blurred_test_1.jpg").unwrap();

        tx.send(()).unwrap();
    });

    assert_eq!(rx.recv_timeout(Duration::from_secs(30)), Ok(()))
}

#[test]
fn big_radius() {
    let img = image::open("assets/bg.jpg").unwrap().to_rgb8();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let img_blurred = blur(10, 10.0, img);
        img_blurred.save("assets/blurred_test_2.jpg").unwrap();

        tx.send(()).unwrap();
    });

    assert_eq!(
        rx.recv_timeout(Duration::from_secs(30)),
        Err(mpsc::RecvTimeoutError::Timeout)
    );
}

#[test]
fn async_small_radius() {
    let img = image::open("assets/bg.jpg").unwrap().to_rgb8();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let img_blurred = blur_async(3, 10.0, img);

        img_blurred.save("assets/blurred_test_3.jpg").unwrap();

        tx.send(()).unwrap();
    });

    assert_eq!(rx.recv_timeout(Duration::from_secs(30)), Ok(()));
}

#[test]
fn async_big_radius() {
    let img = image::open("assets/bg.jpg").unwrap().to_rgb8();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let img_blurred = blur_async(10, 10.0, img);

        img_blurred.save("assets/blurred_test_3.jpg").unwrap();

        tx.send(()).unwrap();
    });

    assert_eq!(rx.recv_timeout(Duration::from_secs(30)), Ok(()));
}
