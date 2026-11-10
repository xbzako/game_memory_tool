## [0.2.0] - 2026-11-10
### Changed
Changed the type of many function or method address parameters from usize to Address<br>
修改了许多函数或方法地址参数 usize 类型为 Address 类型

Added a lot of trait implementations for the Address type in conversations.rs<br>
增加了 conversations.rs 里面实现了许多关于 Address 类型的 trait

### Fixed
Fixed a crash caused by the feature code scanning method mistakenly thinking some addresses were readable<br>
修复了特征码扫描方法对于某些地址被错误地认为可读导致的崩溃