//! Integration tests for Vantis Grid

use vantis_grid::{Grid, CellValue, NeuralEngine, FormulaEngine, CollaborationManager, ExportFormat};
use std::path::PathBuf;

#[test]
fn test_grid_basic_operations() {
    let grid = Grid::new("Test Workbook".to_string());
    
    // Set cell values
    grid.set_cell_value(0, 0, CellValue::Number(10.0)).unwrap();
    grid.set_cell_value(0, 1, CellValue::Number(20.0)).unwrap();
    grid.set_cell_value(0, 2, CellValue::Text("Hello".to_string())).unwrap();
    
    // Get cell values
    let value1 = grid.get_cell_value(0, 0).unwrap();
    assert_eq!(value1, CellValue::Number(10.0));
    
    let value2 = grid.get_cell_value(0, 1).unwrap();
    assert_eq!(value2, CellValue::Number(20.0));
    
    let value3 = grid.get_cell_value(0, 2).unwrap();
    assert_eq!(value3, CellValue::Text("Hello".to_string()));
}

#[test]
fn test_grid_with_multiple_worksheets() {
    let grid = Grid::with_worksheets(
        "Multi-sheet Workbook".to_string(),
        vec!["Sheet1".to_string(), "Sheet2".to_string(), "Sheet3".to_string()]
    );
    
    let workbook = grid.get_workbook();
    let workbook = workbook.read().unwrap();
    
    assert_eq!(workbook.worksheets.len(), 3);
    assert_eq!(workbook.worksheets[0].name, "Sheet1");
    assert_eq!(workbook.worksheets[1].name, "Sheet2");
    assert_eq!(workbook.worksheets[2].name, "Sheet3");
}

#[test]
fn test_neural_engine_trend_analysis() {
    let engine = NeuralEngine::new();
    
    // Test increasing trend
    let increasing_data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];
    let trend = engine.analyze_trends(&increasing_data).unwrap();
    
    assert!(trend.slope > 0.0);
    assert!(trend.r_squared > 0.9);
    assert_eq!(trend.data_points, 10);
    
    // Test decreasing trend
    let decreasing_data = vec![10.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0];
    let trend = engine.analyze_trends(&decreasing_data).unwrap();
    
    assert!(trend.slope < 0.0);
    assert!(trend.r_squared > 0.9);
}

#[test]
fn test_neural_engine_prediction() {
    let engine = NeuralEngine::new();
    
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    engine.train_model("test_model".to_string(), &data).unwrap();
    
    let prediction = engine.predict("test_model", &[5.0]).unwrap();
    assert!((prediction - 6.0).abs() < 0.5);
}

#[test]
fn test_neural_engine_anomaly_detection() {
    let engine = NeuralEngine::new();
    
    let data_with_anomaly = vec![1.0, 2.0, 3.0, 100.0, 5.0, 6.0, 7.0];
    let trend = engine.analyze_trends(&data_with_anomaly).unwrap();
    
    assert!(!trend.anomalies.is_empty());
    assert!(trend.anomalies.iter().any(|a| a.index == 3));
}

#[test]
fn test_neural_engine_suggestions() {
    let engine = NeuralEngine::new();
    
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let suggestions = engine.generate_suggestions(&data).unwrap();
    
    assert!(!suggestions.is_empty());
}

#[test]
fn test_formula_engine_basic() {
    let engine = FormulaEngine::new();
    
    struct TestContext;
    impl vantis_grid::formulas::EvaluationContext for TestContext {
        fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, vantis_grid::formulas::FormulaError> {
            Ok(CellValue::Number((row + column) as f64))
        }
    }
    
    let context = TestContext;
    
    // Test arithmetic
    let result = engine.evaluate("=1+2", &context).unwrap();
    assert_eq!(result, CellValue::Number(3.0));
    
    let result = engine.evaluate("=10-5", &context).unwrap();
    assert_eq!(result, CellValue::Number(5.0));
    
    let result = engine.evaluate("=3*4", &context).unwrap();
    assert_eq!(result, CellValue::Number(12.0));
    
    let result = engine.evaluate("=20/4", &context).unwrap();
    assert_eq!(result, CellValue::Number(5.0));
}

#[test]
fn test_formula_engine_functions() {
    let engine = FormulaEngine::new();
    
    struct TestContext;
    impl vantis_grid::formulas::EvaluationContext for TestContext {
        fn get_cell_value(&self, _row: usize, _column: usize) -> Result<CellValue, vantis_grid::formulas::FormulaError> {
            Ok(CellValue::Number(0.0))
        }
    }
    
    let context = TestContext;
    
    // Test SUM function
    let result = engine.evaluate("=SUM(1,2,3,4,5)", &context).unwrap();
    assert_eq!(result, CellValue::Number(15.0));
    
    // Test AVERAGE function
    let result = engine.evaluate("=AVERAGE(10,20,30)", &context).unwrap();
    assert_eq!(result, CellValue::Number(20.0));
    
    // Test MIN function
    let result = engine.evaluate("=MIN(5,3,8,1,9)", &context).unwrap();
    assert_eq!(result, CellValue::Number(1.0));
    
    // Test MAX function
    let result = engine.evaluate("=MAX(5,3,8,1,9)", &context).unwrap();
    assert_eq!(result, CellValue::Number(9.0));
    
    // Test COUNT function
    let result = engine.evaluate("=COUNT(1,2,3,4,5)", &context).unwrap();
    assert_eq!(result, CellValue::Number(5.0));
}

#[test]
fn test_formula_engine_logical() {
    let engine = FormulaEngine::new();
    
    struct TestContext;
    impl vantis_grid::formulas::EvaluationContext for TestContext {
        fn get_cell_value(&self, _row: usize, _column: usize) -> Result<CellValue, vantis_grid::formulas::FormulaError> {
            Ok(CellValue::Number(0.0))
        }
    }
    
    let context = TestContext;
    
    // Test IF function
    let result = engine.evaluate("=IF(TRUE, 10, 20)", &context).unwrap();
    assert_eq!(result, CellValue::Number(10.0));
    
    let result = engine.evaluate("=IF(FALSE, 10, 20)", &context).unwrap();
    assert_eq!(result, CellValue::Number(20.0));
    
    // Test AND function
    let result = engine.evaluate("=AND(TRUE, TRUE)", &context).unwrap();
    assert_eq!(result, CellValue::Boolean(true));
    
    let result = engine.evaluate("=AND(TRUE, FALSE)", &context).unwrap();
    assert_eq!(result, CellValue::Boolean(false));
    
    // Test OR function
    let result = engine.evaluate("=OR(TRUE, FALSE)", &context).unwrap();
    assert_eq!(result, CellValue::Boolean(true));
    
    let result = engine.evaluate("=OR(FALSE, FALSE)", &context).unwrap();
    assert_eq!(result, CellValue::Boolean(false));
    
    // Test NOT function
    let result = engine.evaluate("=NOT(TRUE)", &context).unwrap();
    assert_eq!(result, CellValue::Boolean(false));
}

#[test]
fn test_formula_engine_text() {
    let engine = FormulaEngine::new();
    
    struct TestContext;
    impl vantis_grid::formulas::EvaluationContext for TestContext {
        fn get_cell_value(&self, _row: usize, _column: usize) -> Result<CellValue, vantis_grid::formulas::FormulaError> {
            Ok(CellValue::Number(0.0))
        }
    }
    
    let context = TestContext;
    
    // Test CONCATENATE function
    let result = engine.evaluate("=CONCATENATE(&quot;Hello&quot;, &quot; &quot;, &quot;World&quot;)", &context).unwrap();
    assert_eq!(result, CellValue::Text("Hello World".to_string()));
    
    // Test LEN function
    let result = engine.evaluate("=LEN(&quot;Hello&quot;)", &context).unwrap();
    assert_eq!(result, CellValue::Number(5.0));
    
    // Test UPPER function
    let result = engine.evaluate("=UPPER(&quot;hello&quot;)", &context).unwrap();
    assert_eq!(result, CellValue::Text("HELLO".to_string()));
    
    // Test LOWER function
    let result = engine.evaluate("=LOWER(&quot;HELLO&quot;)", &context).unwrap();
    assert_eq!(result, CellValue::Text("hello".to_string()));
}

#[test]
fn test_collaboration_session() {
    let manager = CollaborationManager::new();
    
    // Create session
    let session_id = manager.create_session("doc1".to_string()).unwrap();
    assert!(!session_id.is_empty());
    
    // Join users
    let token1 = manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()).unwrap();
    let token2 = manager.join_session(session_id.clone(), "user2".to_string(), "Bob".to_string()).unwrap();
    
    assert_eq!(token1.user_id, "user1");
    assert_eq!(token2.user_id, "user2");
    
    // Get active users
    let users = manager.get_active_users(session_id.clone()).unwrap();
    assert_eq!(users.len(), 2);
    
    // Get session stats
    let stats = manager.get_session_stats(session_id.clone()).unwrap();
    assert_eq!(stats.active_users, 2);
}

#[test]
fn test_collaboration_changes() {
    let manager = CollaborationManager::new();
    
    let session_id = manager.create_session("doc1".to_string()).unwrap();
    manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()).unwrap();
    
    // Apply changes
    let change1 = vantis_grid::collaboration::CellChange::new("user1".to_string(), 0, 0, "100".to_string());
    let applied1 = manager.apply_change(session_id.clone(), change1).unwrap();
    assert_eq!(applied1.change.new_value, "100");
    
    let change2 = vantis_grid::collaboration::CellChange::new("user1".to_string(), 0, 1, "200".to_string());
    let applied2 = manager.apply_change(session_id.clone(), change2).unwrap();
    assert_eq!(applied2.change.new_value, "200");
    
    // Get session stats
    let stats = manager.get_session_stats(session_id.clone()).unwrap();
    assert_eq!(stats.total_changes, 2);
}

#[test]
fn test_export_csv() {
    let grid = Grid::new("Export Test".to_string());
    
    // Add some data
    grid.set_cell_value(0, 0, CellValue::Number(1.0)).unwrap();
    grid.set_cell_value(0, 1, CellValue::Text("Name".to_string())).unwrap();
    grid.set_cell_value(1, 0, CellValue::Number(2.0)).unwrap();
    grid.set_cell_value(1, 1, CellValue::Text("Value".to_string())).unwrap();
    
    let workbook = grid.get_workbook();
    let workbook = workbook.read().unwrap();
    
    let temp_path = PathBuf::from("/tmp/test_grid_export.csv");
    let exporter = vantis_grid::export::Exporter::new(ExportFormat::Csv);
    
    let result = exporter.export(&workbook, &temp_path);
    assert!(result.is_ok());
    
    // Verify file was created
    assert!(temp_path.exists());
}

#[test]
fn test_export_json() {
    let grid = Grid::new("Export Test".to_string());
    
    // Add some data
    grid.set_cell_value(0, 0, CellValue::Number(42.0)).unwrap();
    grid.set_cell_value(0, 1, CellValue::Text("Test".to_string())).unwrap();
    
    let workbook = grid.get_workbook();
    let workbook = workbook.read().unwrap();
    
    let temp_path = PathBuf::from("/tmp/test_grid_export.json");
    let exporter = vantis_grid::export::Exporter::new(ExportFormat::Json);
    
    let result = exporter.export(&workbook, &temp_path);
    assert!(result.is_ok());
    
    // Verify file was created
    assert!(temp_path.exists());
}

#[test]
fn test_grid_initialization() {
    let result = vantis_grid::init();
    assert!(result.is_ok());
}

#[test]
fn test_neural_engine_initialization() {
    let result = vantis_grid::engine::init();
    assert!(result.is_ok());
}

#[test]
fn test_formula_engine_initialization() {
    let result = vantis_grid::formulas::init();
    assert!(result.is_ok());
}

#[test]
fn test_collaboration_initialization() {
    let result = vantis_grid::collaboration::init();
    assert!(result.is_ok());
}

#[test]
fn test_export_initialization() {
    let result = vantis_grid::export::init();
    assert!(result.is_ok());
}