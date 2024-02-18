slint::include_modules!();

const TAX_PER: f64 = 0.08;
const TIP_PER: f64 = 0.15;
const PROFIT_PER: f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();

        let tax: f64 = num * TAX_PER;
        let tip: f64 = num * TIP_PER;
        let profit: f64 = num * PROFIT_PER;
        let total: f64 = num + tax + tip + profit;
        let result: String = format!(
            "Tax: ${:.2}\nTip: ${:.2}\nProfit: ${:.2}\nTotal: ${:.2}",
            tax, tip, profit, total
        );
        ui.set_results(result.into());
    });

    ui.run()
}
