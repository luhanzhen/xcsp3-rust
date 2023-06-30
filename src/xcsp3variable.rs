/**
 * <p>@project_name: XCSP3-Rust
 * <p/>
 * <p>@author: luhanzhen
 * <p/>
 * <p>@date: 2023/6/30
 * <p/>
 * <p>@time: 13:47
 * <p/>
 * <p>@this_file_name:XCSP3Variable
 * <p/>
 */

struct XEntity {
    id: String,
}

impl XEntity {
    pub fn new(id: String) -> XEntity {
        XEntity { id }
    }

    pub fn drop(&mut self) {}

    pub fn fake(&mut self) {}
}
