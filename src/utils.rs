pub trait PushIf<T> {
    fn push_if(&mut self, condition: bool, value: T);
    fn push_if_or(&mut self, condition: bool, value: T, alternative: T);
}

impl<T> PushIf<T> for Vec<T> {
    fn push_if(&mut self, condition: bool, value: T) {
        if condition {
            self.push(value);
        }
    }

    fn push_if_or(&mut self, condition: bool, value: T, alternative: T) {
        self.push(if condition { value } else { alternative });
    }
}
