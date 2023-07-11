pub mod guns{
trait Gun{
    fn safety_circle(&self) -> i32;
    fn max_range(&self) -> i32;
    fn calibre(&self) -> i8;
}

impl M777 for Gun{
    fn safety_circle(&self) -> i32{
        return 1240;
    }
    fn max_range(&self) -> i32{
        return 24700;
    }
    fn calibre(&self) -> i8{
        return 155;
    }
}

impl C3 for Gun{
    fn safety_circle(&self) -> i32{
        return 800;
    }
    fn max_range(&self) -> i32{
        return 11270;
    }
    fn calibre(&self) -> i8{
        return 105;
    }
}

impl LG1 for Gun{
    fn safety_circle(&self) -> i32{
        return 840;
    }
    fn max_range(&self) -> i32{
        return 17000;
    }
    fn calibre(&self) -> i8{
        return 105;
    }
}
}