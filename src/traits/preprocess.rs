

pub trait PreProcess {
    type Output;

    fn preprocess(&self) -> Self::Output;
    fn pp(&self) -> Self::Output {
        self.preprocess()
    }
}