use failure::Fallible;

pub struct Cached<T> {
    value: Option<T>,
    last_used: std::time::Instant,
    cache_duration: std::time::Duration,
}

impl<T> Cached<T> {
    pub fn new(duration: impl Into<std::time::Duration>) -> Self {
        Cached {
            value: None,
            last_used: std::time::Instant::now(),
            cache_duration: duration.into(),
        }
    }

    pub fn evict_overdue(&mut self) {
        if self.last_used + self.cache_duration > std::time::Instant::now() {
            self.value = None;
        }
    }

    pub fn get_or_make(&mut self, construct: impl FnOnce() -> T) -> &mut T {
        if self.value.is_none() {
            self.value = Some(construct());
        }
        self.last_used = std::time::Instant::now();
        self.value.as_mut().unwrap()
    }

    pub fn try_get_or_make(&mut self, construct: impl FnOnce() -> Fallible<T>) -> Fallible<&mut T> {
        if self.value.is_none() {
            self.value = Some(construct()?);
        }
        self.last_used = std::time::Instant::now();
        Ok(self.value.as_mut().unwrap())
    }

    pub fn get(&mut self) -> Option<&mut T> {
        if self.value.is_some() {
            self.last_used = std::time::Instant::now();
        }
        self.value.as_mut()
    }

    pub fn peek(&self) -> Option<&T> {
        self.value.as_ref()
    }
}
