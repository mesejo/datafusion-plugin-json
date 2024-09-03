use datafusion::execution::context::SessionContext;
use datafusion_functions_json::register_all;
use datafusion_plugin_core::{Function, InvocationError, PluginRegistrar};

datafusion_plugin_core::export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("json_functions", Box::new(JsonFunctions));
}

#[derive(Debug, Clone, PartialEq)]
pub struct JsonFunctions;

impl Function for JsonFunctions {
    fn call(&self, context: &mut SessionContext) -> Result<(), InvocationError> {
        register_all(context).map_err(|e| InvocationError::from(e))
    }
}
