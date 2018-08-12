pub struct EnvModuleResolver;

use wasmi::ModuleImportResolver;

use wasmi::GlobalInstance;

use wasmi::FuncRef;
use wasmi::GlobalDescriptor;
use wasmi::GlobalRef;
use wasmi::MemoryDescriptor;
use wasmi::MemoryRef;
use wasmi::RuntimeValue;
use wasmi::Signature;
use wasmi::TableDescriptor;
use wasmi::TableRef;

use wasmi;

impl ModuleImportResolver for EnvModuleResolver {
    /// Resolve a function.
    ///
    /// Returned function should match given `signature`, i.e. all parameter types and return value should have exact match.
    /// Otherwise, link-time error will occur.
    fn resolve_func(
        &self,
        _field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, wasmi::Error> {
        //println!("func {:?}", field_name);

        Err(wasmi::Error::Memory("Does not exist".to_string()))
    }

    /// Resolve a global variable.
    ///
    /// Returned global should match given `descriptor`, i.e. type and mutability
    /// should match. Otherwise, link-time error will occur.
    fn resolve_global(
        &self,
        field_name: &str,
        _descriptor: &GlobalDescriptor,
    ) -> Result<GlobalRef, wasmi::Error> {
        //println!("global {:?}", field_name);

        if field_name.contains("memoryBase") {
            return Ok(GlobalInstance::alloc(RuntimeValue::I32(0), false));
        }

        Err(wasmi::Error::Global("Does not exist".to_string()))
    }

    /// Resolve a memory.
    ///
    /// Returned memory should match requested memory (described by the `descriptor`),
    /// i.e. initial size of a returned memory should be equal or larger than requested memory.
    /// Furthermore, if requested memory have maximum size, returned memory either should have
    /// equal or larger maximum size or have no maximum size at all.
    /// If returned memory doesn't match the requested then link-time error will occur.
    fn resolve_memory(
        &self,
        _field_name: &str,
        _descriptor: &MemoryDescriptor,
    ) -> Result<MemoryRef, wasmi::Error> {
        Err(wasmi::Error::Memory("Does not exist".to_string()))
    }

    /// Resolve a table.
    ///
    /// Returned table should match requested table (described by the `descriptor`),
    /// i.e. initial size of a returned table should be equal or larger than requested table.
    /// Furthermore, if requested memory have maximum size, returned memory either should have
    /// equal or larger maximum size or have no maximum size at all.
    /// If returned table doesn't match the requested then link-time error will occur.
    fn resolve_table(
        &self,
        _field_name: &str,
        _descriptor: &TableDescriptor,
    ) -> Result<TableRef, wasmi::Error> {
        //println!("table {:?}", field_name);

        Err(wasmi::Error::Table("Does not exist".to_string()))
    }
}
