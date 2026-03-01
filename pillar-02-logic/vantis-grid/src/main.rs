//! Vantis Grid - AI-powered spreadsheet application
//! 
//! Main entry point for the Vantis Grid application

use vantis_grid::{Grid, CellValue, NeuralEngine, FormulaEngine, CollaborationManager, ExportFormat};
use std::path::PathBuf;

fn main() {
    println!("Vantis Grid v{}", vantis_grid::VERSION);
    println!("AI-powered spreadsheet for VantisOffice\n");
    
    // Initialize subsystems
    if let Err(e) = vantis_grid::init() {
        eprintln!("Initialization error: {}", e);
        std::process::exit(1);
    }
    
    println!("✓ Vantis Grid initialized successfully\n");
    
    // Create a new workbook
    let grid = Grid::new("Demo Workbook".to_string());
    println!("✓ Created new workbook: Demo Workbook\n");
    
    // Add some sample data
    println!("Adding sample data...");
    grid.set_cell_value(0, 0, CellValue::Text("Month".to_string())).unwrap();
    grid.set_cell_value(0, 1, CellValue::Text("Sales".to_string())).unwrap();
    grid.set_cell_value(0, 2, CellValue::Text("Growth".to_string())).unwrap();
    
    grid.set_cell_value(1, 0, CellValue::Text("January".to_string())).unwrap();
    grid.set_cell_value(1, 1, CellValue::Number(10000.0)).unwrap();
    grid.set_cell_value(1, 2, CellValue::Number(0.05)).unwrap();
    
    grid.set_cell_value(2, 0, CellValue::Text("February".to_string())).unwrap();
    grid.set_cell_value(2, 1, CellValue::Number(12000.0)).unwrap();
    grid.set_cell_value(2, 2, CellValue::Number(0.20)).unwrap();
    
    grid.set_cell_value(3, 0, CellValue::Text("March".to_string())).unwrap();
    grid.set_cell_value(3, 1, CellValue::Number(15000.0)).unwrap();
    grid.set_cell_value(3, 2, CellValue::Number(0.25)).unwrap();
    
    grid.set_cell_value(4, 0, CellValue::Text("April".to_string())).unwrap();
    grid.set_cell_value(4, 1, CellValue::Number(18000.0)).unwrap();
    grid.set_cell_value(4, 2, CellValue::Number(0.20)).unwrap();
    
    grid.set_cell_value(5, 0, CellValue::Text("May".to_string())).unwrap();
    grid.set_cell_value(5, 1, CellValue::Number(22000.0)).unwrap();
    grid.set_cell_value(5, 2, CellValue::Number(0.22)).unwrap();
    
    println!("✓ Sample data added\n");
    
    // Display the data
    println!("Current data:");
    println!("─────────────────────────────────");
    for row in 0..=5 {
        for col in 0..=2 {
            if let Ok(value) = grid.get_cell_value(row, col) {
                let display = match value {
                    CellValue::Number(n) => format!("{:.2}", n),
                    CellValue::Text(s) => format!("{:<12}", s),
                    CellValue::Boolean(b) => b.to_string(),
                    CellValue::Empty => String::new(),
                    _ => String::new(),
                };
                print!("{} ", display);
            }
        }
        println!();
    }
    println!("─────────────────────────────────\n");
    
    // Test Neural Engine - Trend Analysis
    println!("Testing Neural Engine - Trend Analysis:");
    let mut engine = NeuralEngine::new();
    let sales_data = vec![10000.0, 12000.0, 15000.0, 18000.0, 22000.0];
    
    match engine.analyze_trends(&sales_data) {
        Ok(trend) => {
            println!("  Trend Direction: {:?}", trend.trend_direction);
            println!("  Slope: {:.2}", trend.slope);
            println!("  R-squared: {:.4}", trend.r_squared);
            println!("  Confidence: {:.2}%", trend.confidence * 100.0);
            
            if !trend.anomalies.is_empty() {
                println!("  Anomalies detected: {}", trend.anomalies.len());
            } else {
                println!("  No anomalies detected");
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // Test Neural Engine - Prediction
    println!("Testing Neural Engine - Prediction:");
    match engine.train_model("sales_model".to_string(), &sales_data) {
        Ok(_) => {
            match engine.predict("sales_model", &[22000.0]) {
                Ok(prediction) => {
                    println!("  Predicted next month sales: {:.2}", prediction);
                }
                Err(e) => println!("  Error: {}", e),
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // Test Neural Engine - Suggestions
    println!("Testing Neural Engine - AI Suggestions:");
    match engine.generate_suggestions(&sales_data) {
        Ok(suggestions) => {
            for suggestion in suggestions {
                println!("  • {}", suggestion.message);
            }
        }
        Err(e) => println!("  Error: {}", e),
    }
    println!();
    
    // Test Formula Engine
    println!("Testing Formula Engine:");
    let formula_engine = FormulaEngine::new();
    
    struct TestContext;
    impl vantis_grid::formulas::EvaluationContext for TestContext {
        fn get_cell_value(&self, row: usize, column: usize) -> Result<CellValue, vantis_grid::formulas::FormulaError> {
            Ok(CellValue::Number((row + column) as f64))
        }
    }
    
    let context = TestContext;
    
    let formulas = vec![
        "=SUM(10000, 12000, 15000, 18000, 22000)",
        "=AVERAGE(10000, 12000, 15000, 18000, 22000)",
        "=MAX(10000, 12000, 15000, 18000, 22000)",
        "=MIN(10000, 12000, 15000, 18000, 22000)",
    ];
    
    for formula in formulas {
        match formula_engine.evaluate(formula, &context) {
            Ok(result) => {
                let value_str = match result {
                    CellValue::Number(n) => format!("{:.2}", n),
                    _ => format!("{:?}", result),
                };
                println!("  {} = {}", formula, value_str);
            }
            Err(e) => println!("  {} = Error: {}", formula, e),
        }
    }
    println!();
    
    // Test Collaboration
    println!("Testing Collaboration Manager:");
    let collab_manager = CollaborationManager::new();
    
    match collab_manager.create_session("demo_doc".to_string()) {
        Ok(session_id) => {
            println!("  ✓ Created collaboration session: {}", session_id);
            
            match collab_manager.join_session(session_id.clone(), "user1".to_string(), "Alice".to_string()) {
                Ok(_) => println!("  ✓ User 'Alice' joined the session"),
                Err(e) => println!("  ✗ Error joining session: {}", e),
            }
            
            match collab_manager.join_session(session_id.clone(), "user2".to_string(), "Bob".to_string()) {
                Ok(_) => println!("  ✓ User 'Bob' joined the session"),
                Err(e) => println!("  ✗ Error joining session: {}", e),
            }
            
            match collab_manager.get_active_users(session_id.clone()) {
                Ok(users) => println!("  ✓ Active users: {}", users.len()),
                Err(e) => println!("  ✗ Error getting active users: {}", e),
            }
        }
        Err(e) => println!("  ✗ Error creating session: {}", e),
    }
    println!();
    
    // Test Export
    println!("Testing Export:");
    let workbook = grid.get_workbook();
    let workbook = workbook.read().unwrap();
    
    let csv_path = PathBuf::from("/tmp/demo_workbook.csv");
    let exporter = vantis_grid::export::Exporter::new(ExportFormat::Csv);
    
    match exporter.export(&workbook, &csv_path) {
        Ok(_) => println!("  ✓ Exported to CSV: {}", csv_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }
    
    let json_path = PathBuf::from("/tmp/demo_workbook.json");
    let json_exporter = vantis_grid::export::Exporter::new(ExportFormat::Json);
    
    match json_exporter.export(&workbook, &json_path) {
        Ok(_) => println!("  ✓ Exported to JSON: {}", json_path.display()),
        Err(e) => println!("  ✗ Export error: {}", e),
    }
    println!();
    
    println!("─────────────────────────────────");
    println!("Vantis Grid demo completed successfully!");
    println!("─────────────────────────────────");
}