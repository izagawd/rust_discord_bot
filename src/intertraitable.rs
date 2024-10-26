use std::any::Any;


trait Intertraitable : Any{

}
trait Casters : Any{



}

impl<T: Intertraitable> Casters for  T{

}