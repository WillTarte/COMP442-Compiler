use crate::semantics::symbol_table::{SymbolTable, Type, Scope, FunctionEntry, ClassEntry, VariableEntry, ParameterEntry};

pub fn sizeof(t: &Type, symbols: &SymbolTable) -> u32
{
    let size: u32 = match t
    {
        Type::Integer => { 4 }
        Type::IntegerArray(dim) => {
            4u32 * dim.iter().product::<u32>()
        }
        Type::Float => { 8 }
        Type::FloatArray(dim) => {
            8u32 * dim.iter().product::<u32>()
        }
        Type::String => { todo!() }
        Type::StringArray(dim) => {
            0u32 * dim.iter().product::<u32>()
        }
        Type::Custom(ident) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident)
            {
                for scope in ce.table().scopes()
                {
                    if let Scope::Variable(ve) = scope
                    {
                        temp_size += sizeof(ve.var_type(), symbols);
                    }
                    else if let Scope::Function(fe) = scope
                    {
                        todo!()
                    }
                }
            }
            else { panic!() }
            temp_size

        }
        Type::CustomArray(ident, dim) => {
            let mut temp_size: u32 = 0;
            if let Some(Scope::Class(ce)) = symbols.find_scope_by_ident(ident)
            {
                for scope in ce.table().scopes()
                {
                    if let Scope::Variable(ve) = scope
                    {
                        temp_size += sizeof(ve.var_type(), symbols);
                    }
                    else if let Scope::Function(fe) = scope
                    {
                        todo!()
                    }
                }
            }
            else { panic!() }
            temp_size * dim.iter().product::<u32>()
        }
        Type::Void => { 0 }
    };
    size
}
