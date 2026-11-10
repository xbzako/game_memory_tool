## [0.2.1] - 2026-11-10
### Changed

### Fixed
Corrected the error description in CHANGELOG.md of version 0.2.0 from 'The feature code scanning method adds support for feature codes' to 'The feature code scanning method adds support for wildcards'<br>
修改了 0.2.0 版本 CHANGELOG.md 里的错误描述"特征码扫描方法增加了对特征码的支持" 为 "特征码扫描方法增加了对通配符的支持"


## [0.2.0] - 2026-11-10
### Changed
Changed the type of many function or method address parameters from usize to Address<br>
修改了许多函数或方法地址参数 usize 类型为 Address 类型

Added a lot of trait implementations for the Address type in conversations.rs<br>
增加了 conversations.rs 里面实现了许多关于 Address 类型的 trait

The feature code scanning method now adds support for feature codes<br>
特征码扫描方法增加了对特征码的支持

### Fixed
Fixed a crash caused by the feature code scanning method mistakenly thinking some addresses were readable<br>
修复了特征码扫描方法对于某些地址被错误地认为可读导致的崩溃