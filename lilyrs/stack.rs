




pub struct StrType {
    pub string: String,
    pub name: String,
    pub class: i8,
    pub used: bool,
    pub declared: bool,
  }
  
  pub struct IntType {
    pub num: i64,
    pub name: String,
    pub class: i8,
    pub used: bool,
    pub declared: bool,
  }
  
  
  
  
  pub struct ParameterType {
    pub name: String,
    pub ptype: String,
    pub optional: bool,
    pub elipses: bool,
  }
  
  pub struct FunctionType {
    pub name: String,
    pub returns: String,
    pub parameters: Vec<ParameterType>,
  
    pub used: bool,
    pub declared: bool,
  }
  
  pub struct Stack {
    pub str_stack: Vec<StrType>,
    pub int_stack: Vec<IntType>,
  }