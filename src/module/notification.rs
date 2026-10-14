use rodio;

pub struct Notification {
    _stream: rodio::OutputStream, // 鳴らしている間は保持しておく必要がある
    handle: rodio::OutputStreamHandle,
}

impl Notification {
    pub fn new() -> Option<Self> {
        let (stream, handle) = rodio::OutputStream::try_default().ok()?;
        Some(Self { _stream: stream, handle })
    }

    /// 「ピンポン」の2音
    pub fn chime(&self) {
        use rodio::{source::SineWave, Sink, Source};
        let Ok(sink) = Sink::try_new(&self.handle) else {
            return;
        };
        sink.append(
            SineWave::new(880.0)
                .take_duration(std::time::Duration::from_millis(150))
                .amplify(0.25),
        );
        sink.append(
            SineWave::new(1175.0)
                .take_duration(std::time::Duration::from_millis(300))
                .amplify(0.25),
        );
        sink.detach(); // 再生はバックグラウンドで続く
    }
}
