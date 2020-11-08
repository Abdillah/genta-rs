pub mod ffi;

pub struct Connection(*mut ffi::TrackerSparqlConnection);

impl Connection {
    pub fn new() -> Result<Self, String> {
        let mut error: std::mem::MaybeUninit<*mut ffi::GError> = std::mem::MaybeUninit::new(std::ptr::null_mut());
        let connection = {
            let connection = unsafe { ffi::tracker_sparql_connection_get(std::ptr::null_mut(), error.as_mut_ptr()) };
            if connection.is_null() {
                None
            } else {
                Some(connection)
            }
        };

        let error = unsafe { *error.as_ptr() };
        if !error.is_null() {
            let errstr = unsafe { std::ffi::CStr::from_ptr((*error).message).to_str().unwrap().to_string() };
            Err(errstr)
        } else if connection.is_none() {
            Err("Unknown error".to_string())
        } else {
            Ok(Self(connection.unwrap()))
        }
    }

    pub fn query(self, q: String) -> Result<Cursor, String> {
        let mut error: std::mem::MaybeUninit<*mut ffi::GError> = std::mem::MaybeUninit::new(std::ptr::null_mut());
        let cursor = unsafe {
            ffi::tracker_sparql_connection_query (self.0, q.as_ptr() as *const libc::c_char, std::ptr::null_mut(), error.as_mut_ptr())
        };

        let error = unsafe { *error.as_ptr() };
        if !error.is_null() {
            let errstr = unsafe { std::ffi::CStr::from_ptr((*error).message).to_str().unwrap().to_string() };
            Err(errstr)
        } else if cursor.is_null() {
            Err("No cursor created".to_string())
        } else {
            Ok(Cursor(cursor))
        }
    }
}

pub struct Cursor(*mut ffi::TrackerSparqlCursor);

impl std::iter::Iterator for Cursor {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut error: std::mem::MaybeUninit<*mut ffi::GError> = std::mem::MaybeUninit::new(std::ptr::null_mut());
        let b = unsafe { ffi::tracker_sparql_cursor_next(self.0, std::ptr::null_mut(), error.as_mut_ptr()) };
        if b != 0 {
            let mut result = vec!();
            unsafe {
                let n = ffi::tracker_sparql_cursor_get_n_columns(self.0);
                for i in 0..n {
                    let s = ffi::tracker_sparql_cursor_get_string (self.0, i, std::ptr::null_mut());
                    let s = std::ffi::CStr::from_ptr(s).to_str().unwrap().to_string();
                    result.push(s);
                }
            };
            Some(result)
        } else {
            None
        }
    }
}