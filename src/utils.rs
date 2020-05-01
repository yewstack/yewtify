use yew::Classes;

// TODO: Move to `Classes` in `yew`.
pub trait PushIf<T> {
    fn push_if(&mut self, condition: bool, value: T);
    fn push_if_or(&mut self, condition: bool, value: T, alternative: T);
}

// TODO: Remove
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

impl<T: AsRef<str>> PushIf<T> for Classes {
    fn push_if(&mut self, condition: bool, value: T) {
        if condition {
            self.push(value.as_ref());
        }
    }

    fn push_if_or(&mut self, condition: bool, value: T, alternative: T) {
        self.push(if condition {
            value.as_ref()
        } else {
            alternative.as_ref()
        });
    }
}
