# CZ-DSL technical documentation

The Chinese originals in `zh/` are authoritative; the English files in `en/`
are faithful translations of them (same section order, same tables and code
blocks).

| # | 中文 (zh/) | English (en/) | Scope |
|---|---|---|---|
| 01 | `01_语言规范.md` | `01_Language_Specification.md` | grammar (EBNF), 43 atoms / 7 groups, grade system, four-level classification, unit literals, constraints, compatibility mapping |
| 02 | `02_设计原则.md` | `02_Design_Principles.md` | minimal atomicity, completeness, reversibility, coordinate independence; composition by interface contract |
| 03 | `03_形式化验证.md` | `03_Formal_Verification.md` | the five propositions (reversibility, soundness, injectivity, transferability, extension homomorphism) with proof sketches and stated boundaries |
| 04 | `04_工程化协议.md` | `04_Engineering_Protocol.md` | toolchain contract, validator passes and diagnostic codes, compilation pathway, benchmark protocol |
| 06 | `06_工具集测试报告.md` | `06_Tool_Test_Report.md` | smoke / compatibility / stability test results |

*Paper drafts and submission material are intentionally not part of this
repository.*

# 中文说明

`zh/` 为中文原件（权威版本），`en/` 为对应英文翻译，章节、表格与代码块一一对应。
文档编号 05、07、08 为论文稿与投稿材料，不随本开源库发布。
