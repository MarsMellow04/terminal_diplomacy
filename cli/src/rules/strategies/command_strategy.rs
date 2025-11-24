use diplomacy::order::MainCommand;

trait CommandStrategy {
    fn legal_destinations(
        &self,
        unit: &UnitPosition<'static, RegionKey>
    ) -> Vec<MainCommand>;
}