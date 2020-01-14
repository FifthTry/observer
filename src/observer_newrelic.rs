use failure::_core::cell::RefCell;

pub struct ObserverNewRelic {
    segment_stack: RefCell<Vec<ackorelic::acko_segment::Segment>>,
}

impl ObserverNewRelic {
    fn new() -> Self {
        ObserverNewRelic {
            segment_stack: RefCell::new(vec![]),
        }
    }
}
/// Implementation of Backend trait for NewRelic

impl crate::Backend for ObserverNewRelic {
    /// This will start NewRelic app
    fn app_started(&self) {}
    /// This will end NewRelic app
    fn app_ended(&self) {}
    /// This method will be called when context has been created.
    fn context_created(&self) {
        // Need to create web transaction of NewRelic
    }
    /// This method will be called when context ended.
    fn context_ended(&self) {
        // Need to end web transaction
    }
    /// This method will be when span created.
    fn span_created(&self) {
        // Need to start a segment and store it somewhere
    }
    /// This method will be when span needs to logged.
    fn span_log(&self) {}
    /// This method will be when span ended.
    fn span_ended(&self) {
        // Needs to end a segment which was stored earlier
    }
}

fn _test() {
    let _t: Box<dyn crate::Backend> = Box::new(ObserverNewRelic::new());
}
