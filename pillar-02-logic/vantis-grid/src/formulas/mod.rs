//! Formula engine for spreadsheet calculations
//! 
//! Supports Excel-compatible formulas with extensions for AI features

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use regex::Regex;
use chrono::Datelike;
use crate::core::{CellValue, parse_cell_reference};

/// Formula engine for evaluating spreadsheet formulas
pub struct FormulaEngine {
    functions: FunctionRegistry,
    cache: Arc<RwLock<HashMap<String, CellValue>>>,
    circular_reference_detection: bool,
}

impl FormulaEngine {
    pub fn new() -> Self {
        FormulaEngine {
            functions: FunctionRegistry::new(),
            cache: Arc::new(RwLock::new(HashMap::new())),
            circular_reference_detection: true,
        }
    }
    
    pub fn with_functions(functions: FunctionRegistry) -> Self {
        FormulaEngine {
            functions,
            cache: Arc::new(RwLock::new(HashMap::new())),
            circular_reference_detection: true,
        }
    }
    
    /// Enable or disable circular reference detection
    pub fn set_circular_reference_detection(&mut self, enabled: bool) {
        self.circular_reference_detection = enabled;
    }
    
    /// Evaluate a formula
    pub fn evaluate(&self, formula: &str, context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        // Remove leading '=' if present
        let formula = formula.trim_start_matches('=').trim();
        
        if formula.is_empty() {
            return Ok(CellValue::Empty);
        }
        
        // Check cache
        if let Ok(cache) = self.cache.read() {
            if let Some(cached) = cache.get(formula) {
                return Ok(cached.clone());
            }
        }
        
        // Parse and evaluate
        let result = self.parse_and_evaluate(formula, context)?;
        
        // Cache result
        if let Ok(mut cache) = self.cache.write() {
            cache.insert(formula.to_string(), result.clone());
        }
        
        Ok(result)
    }
    
    /// Parse and evaluate a formula
    fn parse_and_evaluate(&self, formula: &str, context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        // Check for function calls
        if let Some(function_call) = self.extract_function_call(formula) {
            return self.evaluate_function(&function_call.name, &function_call.args, context);
        }
        
        // Check for cell references
        if let Ok((row, col)) = parse_cell_reference(formula) {
            return context.get_cell_value(row, col);
        }
        
        // Check for ranges
        if formula.contains(':') {
            return self.evaluate_range(formula, context);
        }
        
        // Check for numbers
        if let Ok(num) = formula.parse::<f64>() {
            return Ok(CellValue::Number(num));
        }
        
        // Check for booleans
        match formula.to_uppercase().as_str() {
            "TRUE" => return Ok(CellValue::Boolean(true)),
            "FALSE" => return Ok(CellValue::Boolean(false)),
            _ => {}
        }
        
        // Check for quoted strings
        if formula.starts_with('"') && formula.ends_with('"') {
            let text = &formula[1..formula.len()-1];
            return Ok(CellValue::Text(text.to_string()));
        }
        
        // Try as arithmetic expression
        self.evaluate_arithmetic(formula, context)
    }
    
    /// Extract function call from formula
    fn extract_function_call(&self, formula: &str) -> Option<FunctionCall> {
        let re = Regex::new(r"^([A-Z][A-Z0-9_]*)\((.*)\)$").ok()?;
        let caps = re.captures(formula)?;
        
        let name = caps.get(1)?.as_str().to_string();
        let args_str = caps.get(2)?.as_str();
        
        let args = self.parse_arguments(args_str);
        
        Some(FunctionCall { name, args })
    }
    
    /// Parse function arguments
    fn parse_arguments(&self, args_str: &str) -> Vec<String> {
        let mut args = Vec::new();
        let mut current = String::new();
        let mut paren_depth = 0;
        let mut in_quotes = false;
        
        for c in args_str.chars() {
            match c {
                '"' => {
                    in_quotes = !in_quotes;
                    current.push(c);
                }
                '(' if !in_quotes => {
                    paren_depth += 1;
                    current.push(c);
                }
                ')' if !in_quotes => {
                    paren_depth -= 1;
                    current.push(c);
                }
                ',' if !in_quotes && paren_depth == 0 => {
                    args.push(current.trim().to_string());
                    current = String::new();
                }
                _ => {
                    current.push(c);
                }
            }
        }
        
        if !current.trim().is_empty() {
            args.push(current.trim().to_string());
        }
        
        args
    }
    
    /// Evaluate a function call
    fn evaluate_function(&self, name: &str, args: &[String], context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        let function = self.functions.get(name)
            .ok_or_else(|| FormulaError::UnknownFunction(name.to_string()))?;
        
        // Evaluate arguments
        let evaluated_args: Result<Vec<CellValue>, FormulaError> = args.iter()
            .map(|arg| self.parse_and_evaluate(arg, context))
            .collect();
        
        let evaluated_args = evaluated_args?;
        
        // Call function
        function.call(&evaluated_args, context)
    }
    
    /// Evaluate a range reference
    fn evaluate_range(&self, range: &str, context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        let parts: Vec<&str> = range.split(':').collect();
        if parts.len() != 2 {
            return Err(FormulaError::InvalidRange(range.to_string()));
        }
        
        let start = parse_cell_reference(parts[0])?;
        let end = parse_cell_reference(parts[1])?;
        
        let mut values = Vec::new();
        for row in start.0..=end.0 {
            for col in start.1..=end.1 {
                if let Ok(value) = context.get_cell_value(row, col) {
                    values.push(value);
                }
            }
        }
        
        Ok(CellValue::Text(format!("{:?}", values)))
    }
    
    /// Evaluate arithmetic expression
    fn evaluate_arithmetic(&self, expr: &str, context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        // Simple arithmetic evaluator (can be extended with proper parser)
        let tokens = self.tokenize_arithmetic(expr)?;
        let result = self.evaluate_tokens(&tokens, context)?;
        Ok(result)
    }
    
    /// Tokenize arithmetic expression
    fn tokenize_arithmetic(&self, expr: &str) -> Result<Vec<ArithmeticToken>, FormulaError> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        
        for c in expr.chars() {
            match c {
                '+' | '-' | '*' | '/' | '^' | '(' | ')' => {
                    if !current.is_empty() {
                        tokens.push(self.parse_arithmetic_token(&current)?);
                        current.clear();
                    }
                    tokens.push(ArithmeticToken::Operator(c));
                }
                ' ' => {
                    if !current.is_empty() {
                        tokens.push(self.parse_arithmetic_token(&current)?);
                        current.clear();
                    }
                }
                _ => {
                    current.push(c);
                }
            }
        }
        
        if !current.is_empty() {
            tokens.push(self.parse_arithmetic_token(&current)?);
        }
        
        Ok(tokens)
    }
    
    /// Parse arithmetic token
    fn parse_arithmetic_token(&self, token: &str) -> Result<ArithmeticToken, FormulaError> {
        if let Ok(num) = token.parse::<f64>() {
            return Ok(ArithmeticToken::Number(num));
        }
        
        if let Ok((row, col)) = parse_cell_reference(token) {
            return Ok(ArithmeticToken::CellReference(row, col));
        }
        
        Err(FormulaError::InvalidToken(token.to_string()))
    }
    
    /// Evaluate arithmetic tokens
    fn evaluate_tokens(&self, tokens: &[ArithmeticToken], context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        // Simple evaluator (can be improved with proper shunting-yard algorithm)
        let mut values: Vec<f64> = Vec::new();
        let mut operators: Vec<char> = Vec::new();
        
        for token in tokens {
            match token {
                ArithmeticToken::Number(n) => {
                    values.push(*n);
                }
                ArithmeticToken::CellReference(row, col) => {
                    let value = context.get_cell_value(*row, *col)?;
                    match value {
                        CellValue::Number(n) => values.push(n),
                        _ => return Err(FormulaError::TypeError("Expected number".to_string())),
                    }
                }
                ArithmeticToken::Operator(op) => {
                    while let Some(&prev_op) = operators.last() {
                        if self.precedence(prev_op) >= self.precedence(*op) {
                            self.apply_operator(&mut values, prev_op)?;
                            operators.pop();
                        } else {
                            break;
                        }
                    }
                    operators.push(*op);
                }
            }
        }
        
        while let Some(op) = operators.pop() {
            self.apply_operator(&mut values, op)?;
        }
        
        if values.len() == 1 {
            Ok(CellValue::Number(values[0]))
        } else {
            Err(FormulaError::EvaluationError("Invalid expression".to_string()))
        }
    }
    
    /// Get operator precedence
    fn precedence(&self, op: char) -> i32 {
        match op {
            '+' | '-' => 1,
            '*' | '/' => 2,
            '^' => 3,
            _ => 0,
        }
    }
    
    /// Apply operator to values
    fn apply_operator(&self, values: &mut Vec<f64>, op: char) -> Result<(), FormulaError> {
        if values.len() < 2 {
            return Err(FormulaError::EvaluationError("Insufficient values".to_string()));
        }
        
        let b = values.pop().unwrap();
        let a = values.pop().unwrap();
        
        let result = match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => {
                if b == 0.0 {
                    return Err(FormulaError::DivisionByZero);
                }
                a / b
            }
            '^' => a.powf(b),
            _ => return Err(FormulaError::UnknownOperator(op)),
        };
        
        values.push(result);
        Ok(())
    }
    
    /// Clear formula cache
    pub fn clear_cache(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }
}

impl Default for FormulaEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Function call representation
#[derive(Debug, Clone)]
struct FunctionCall {
    name: String,
    args: Vec<String>,
}

/// Arithmetic token
#[derive(Debug, Clone)]
enum ArithmeticToken {
    Number(f64),
    CellReference(usize, usize),
    Operator(char),
}

/// Evaluation context for formulas
pub trait EvaluationContext {
    fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, FormulaError>;
}

// Implement EvaluationContext for &dyn EvaluationContext
impl<T: EvaluationContext + ?Sized> EvaluationContext for &T {
    fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, FormulaError> {
        (**self).get_cell_value(row, column)
    }
}

/// Formula errors
#[derive(Debug, thiserror::Error)]
pub enum FormulaError {
    #[error("Unknown function: {0}")]
    UnknownFunction(String),
    
    #[error("Invalid range: {0}")]
    InvalidRange(String),
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("Type error: {0}")]
    TypeError(String),
    
    #[error("Evaluation error: {0}")]
    EvaluationError(String),
    
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Unknown operator: {0}")]
    UnknownOperator(char),
    
    #[error("Circular reference detected")]
    CircularReference,
    
    #[error("Invalid argument count for function {0}: expected {1}, got {2}")]
    InvalidArgumentCount(String, usize, usize),

    #[error("{0}")]
    ParseError(String),
}

impl From<String> for FormulaError {
    fn from(s: String) -> Self {
        FormulaError::ParseError(s)
    }
}

/// Function registry for custom functions
pub struct FunctionRegistry {
    functions: HashMap<String, Box<dyn Function>>,
}

impl FunctionRegistry {
    pub fn new() -> Self {
        let mut registry = FunctionRegistry {
            functions: HashMap::new(),
        };
        
        // Register built-in functions
        registry.register_builtins();
        
        registry
    }
    
    fn register_builtins(&mut self) {
        // Math functions
        self.register("SUM", Box::new(SumFunction));
        self.register("AVERAGE", Box::new(AverageFunction));
        self.register("MIN", Box::new(MinFunction));
        self.register("MAX", Box::new(MaxFunction));
        self.register("COUNT", Box::new(CountFunction));
        self.register("ABS", Box::new(AbsFunction));
        self.register("ROUND", Box::new(RoundFunction));
        self.register("POWER", Box::new(PowerFunction));
        self.register("SQRT", Box::new(SqrtFunction));
        
        // Logical functions
        self.register("IF", Box::new(IfFunction));
        self.register("AND", Box::new(AndFunction));
        self.register("OR", Box::new(OrFunction));
        self.register("NOT", Box::new(NotFunction));
        
        // Text functions
        self.register("CONCATENATE", Box::new(ConcatenateFunction));
        self.register("LEN", Box::new(LenFunction));
        self.register("UPPER", Box::new(UpperFunction));
        self.register("LOWER", Box::new(LowerFunction));
        self.register("TRIM", Box::new(TrimFunction));
        
        // Date functions
        self.register("TODAY", Box::new(TodayFunction));
        self.register("NOW", Box::new(NowFunction));
        self.register("YEAR", Box::new(YearFunction));
        self.register("MONTH", Box::new(MonthFunction));
        self.register("DAY", Box::new(DayFunction));
    }
    
    pub fn register<F: Function + 'static>(&mut self, name: &str, function: Box<F>) {
        self.functions.insert(name.to_uppercase(), function);
    }
    
    pub fn get(&self, name: &str) -> Option<&dyn Function> {
        self.functions.get(&name.to_uppercase()).map(|f| f.as_ref())
    }
}

/// Function trait for custom functions
pub trait Function: Send + Sync {
    fn call(&self, args: &[CellValue], context: &dyn EvaluationContext) -> Result<CellValue, FormulaError>;
}

impl<F> Function for F
where
    F: Fn(&[CellValue], &dyn EvaluationContext) -> Result<CellValue, FormulaError> + Send + Sync,
{
    fn call(&self, args: &[CellValue], context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        self(args, context)
    }
}

// Built-in function implementations

struct SumFunction;
impl Function for SumFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        let sum: f64 = args.iter()
            .map(|v| match v {
                CellValue::Number(n) => Ok(*n),
                _ => Err(FormulaError::TypeError("Expected number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        Ok(CellValue::Number(sum))
    }
}

struct AverageFunction;
impl Function for AverageFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("AVERAGE".to_string(), 1, 0));
        }
        
        let sum: f64 = args.iter()
            .map(|v| match v {
                CellValue::Number(n) => Ok(*n),
                _ => Err(FormulaError::TypeError("Expected number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .sum();
        
        Ok(CellValue::Number(sum / args.len() as f64))
    }
}

struct MinFunction;
impl Function for MinFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("MIN".to_string(), 1, 0));
        }
        
        let min = args.iter()
            .map(|v| match v {
                CellValue::Number(n) => Ok(*n),
                _ => Err(FormulaError::TypeError("Expected number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(f64::INFINITY, f64::min);
        
        Ok(CellValue::Number(min))
    }
}

struct MaxFunction;
impl Function for MaxFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("MAX".to_string(), 1, 0));
        }
        
        let max = args.iter()
            .map(|v| match v {
                CellValue::Number(n) => Ok(*n),
                _ => Err(FormulaError::TypeError("Expected number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max);
        
        Ok(CellValue::Number(max))
    }
}

struct CountFunction;
impl Function for CountFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        let count = args.iter()
            .filter(|v| !matches!(v, CellValue::Empty))
            .count();
        Ok(CellValue::Number(count as f64))
    }
}

struct AbsFunction;
impl Function for AbsFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("ABS".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Number(n) => Ok(CellValue::Number(n.abs())),
            _ => Err(FormulaError::TypeError("Expected number".to_string())),
        }
    }
}

struct RoundFunction;
impl Function for RoundFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 && args.len() != 2 {
            return Err(FormulaError::InvalidArgumentCount("ROUND".to_string(), 1, args.len()));
        }
        
        let value = match &args[0] {
            CellValue::Number(n) => *n,
            _ => return Err(FormulaError::TypeError("Expected number".to_string())),
        };
        
        let decimals = if args.len() == 2 {
            match &args[1] {
                CellValue::Number(n) => *n as i32,
                _ => return Err(FormulaError::TypeError("Expected number".to_string())),
            }
        } else {
            0
        };
        
        let multiplier = 10_f64.powi(decimals);
        Ok(CellValue::Number((value * multiplier).round() / multiplier))
    }
}

struct PowerFunction;
impl Function for PowerFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 2 {
            return Err(FormulaError::InvalidArgumentCount("POWER".to_string(), 2, args.len()));
        }
        
        let base = match &args[0] {
            CellValue::Number(n) => *n,
            _ => return Err(FormulaError::TypeError("Expected number".to_string())),
        };
        
        let exponent = match &args[1] {
            CellValue::Number(n) => *n,
            _ => return Err(FormulaError::TypeError("Expected number".to_string())),
        };
        
        Ok(CellValue::Number(base.powf(exponent)))
    }
}

struct SqrtFunction;
impl Function for SqrtFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("SQRT".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Number(n) if *n >= 0.0 => Ok(CellValue::Number(n.sqrt())),
            CellValue::Number(_) => Err(FormulaError::EvaluationError("Cannot calculate square root of negative number".to_string())),
            _ => Err(FormulaError::TypeError("Expected number".to_string())),
        }
    }
}

struct IfFunction;
impl Function for IfFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() < 2 || args.len() > 3 {
            return Err(FormulaError::InvalidArgumentCount("IF".to_string(), 2, args.len()));
        }
        
        let condition = match &args[0] {
            CellValue::Boolean(b) => *b,
            CellValue::Number(n) => *n != 0.0,
            _ => return Err(FormulaError::TypeError("Expected boolean or number".to_string())),
        };
        
        if condition {
            Ok(args[1].clone())
        } else if args.len() == 3 {
            Ok(args[2].clone())
        } else {
            Ok(CellValue::Empty)
        }
    }
}

struct AndFunction;
impl Function for AndFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("AND".to_string(), 1, 0));
        }
        
        let result = args.iter()
            .map(|v| match v {
                CellValue::Boolean(b) => Ok(*b),
                CellValue::Number(n) => Ok(*n != 0.0),
                _ => Err(FormulaError::TypeError("Expected boolean or number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .all(|b| b);
        
        Ok(CellValue::Boolean(result))
    }
}

struct OrFunction;
impl Function for OrFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("OR".to_string(), 1, 0));
        }
        
        let result = args.iter()
            .map(|v| match v {
                CellValue::Boolean(b) => Ok(*b),
                CellValue::Number(n) => Ok(*n != 0.0),
                _ => Err(FormulaError::TypeError("Expected boolean or number".to_string())),
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .any(|b| b);
        
        Ok(CellValue::Boolean(result))
    }
}

struct NotFunction;
impl Function for NotFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("NOT".to_string(), 1, args.len()));
        }
        
        let value = match &args[0] {
            CellValue::Boolean(b) => !b,
            CellValue::Number(n) => *n == 0.0,
            _ => return Err(FormulaError::TypeError("Expected boolean or number".to_string())),
        };
        
        Ok(CellValue::Boolean(value))
    }
}

struct ConcatenateFunction;
impl Function for ConcatenateFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        let result = args.iter()
            .map(|v| match v {
                CellValue::Text(s) => Ok::<String, FormulaError>(s.clone()),
                CellValue::Number(n) => Ok::<String, FormulaError>(n.to_string()),
                CellValue::Boolean(b) => Ok::<String, FormulaError>(if *b { "TRUE".to_string() } else { "FALSE".to_string() }),
                _ => Ok::<String, FormulaError>(String::new()),
            })
            .collect::<Result<Vec<_>, FormulaError>>()?
            .join("");
        
        Ok(CellValue::Text(result))
    }
}

struct LenFunction;
impl Function for LenFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("LEN".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Text(s) => Ok(CellValue::Number(s.len() as f64)),
            _ => Err(FormulaError::TypeError("Expected text".to_string())),
        }
    }
}

struct UpperFunction;
impl Function for UpperFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("UPPER".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Text(s) => Ok(CellValue::Text(s.to_uppercase())),
            _ => Err(FormulaError::TypeError("Expected text".to_string())),
        }
    }
}

struct LowerFunction;
impl Function for LowerFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("LOWER".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Text(s) => Ok(CellValue::Text(s.to_lowercase())),
            _ => Err(FormulaError::TypeError("Expected text".to_string())),
        }
    }
}

struct TrimFunction;
impl Function for TrimFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("TRIM".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Text(s) => Ok(CellValue::Text(s.trim().to_string())),
            _ => Err(FormulaError::TypeError("Expected text".to_string())),
        }
    }
}

struct TodayFunction;
impl Function for TodayFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if !args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("TODAY".to_string(), 0, args.len()));
        }
        
        let today = chrono::Local::now().naive_local().date();
        Ok(CellValue::Date(today))
    }
}

struct NowFunction;
impl Function for NowFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if !args.is_empty() {
            return Err(FormulaError::InvalidArgumentCount("NOW".to_string(), 0, args.len()));
        }
        
        let now = chrono::Local::now().naive_local();
        Ok(CellValue::DateTime(now))
    }
}

struct YearFunction;
impl Function for YearFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("YEAR".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Date(d) => Ok(CellValue::Number(d.year_ce().1 as f64)),
            CellValue::DateTime(dt) => Ok(CellValue::Number(dt.year_ce().1 as f64)),
            _ => Err(FormulaError::TypeError("Expected date".to_string())),
        }
    }
}

struct MonthFunction;
impl Function for MonthFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("MONTH".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Date(d) => Ok(CellValue::Number(d.month() as f64)),
            CellValue::DateTime(dt) => Ok(CellValue::Number(dt.month() as f64)),
            _ => Err(FormulaError::TypeError("Expected date".to_string())),
        }
    }
}

struct DayFunction;
impl Function for DayFunction {
    fn call(&self, args: &[CellValue], _context: &dyn EvaluationContext) -> Result<CellValue, FormulaError> {
        if args.len() != 1 {
            return Err(FormulaError::InvalidArgumentCount("DAY".to_string(), 1, args.len()));
        }
        
        match &args[0] {
            CellValue::Date(d) => Ok(CellValue::Number(d.day() as f64)),
            CellValue::DateTime(dt) => Ok(CellValue::Number(dt.day() as f64)),
            _ => Err(FormulaError::TypeError("Expected date".to_string())),
        }
    }
}

/// Initialize formula engine
pub fn init() -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestContext;
    
    impl EvaluationContext for TestContext {
        fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, FormulaError> {
            Ok(CellValue::Number((row + column) as f64))
        }
    }
    
    #[test]
    fn test_formula_engine_creation() {
        let engine = FormulaEngine::new();
        assert!(engine.functions.get("SUM").is_some());
    }
    
    #[test]
    fn test_arithmetic_evaluation() {
        let engine = FormulaEngine::new();
        let context = TestContext;
        
        let result = engine.evaluate("=1+2", &context).unwrap();
        assert_eq!(result, CellValue::Number(3.0));
    }
    
    #[test]
    fn test_function_evaluation() {
        let engine = FormulaEngine::new();
        let context = TestContext;
        
        let result = engine.evaluate("=SUM(1,2,3)", &context).unwrap();
        assert_eq!(result, CellValue::Number(6.0));
    }
    
    #[test]
    fn test_if_function() {
        let engine = FormulaEngine::new();
        let context = TestContext;
        
        let result = engine.evaluate("=IF(TRUE, 10, 20)", &context).unwrap();
        assert_eq!(result, CellValue::Number(10.0));
    }
}