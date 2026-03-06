# Framework

For coding guidelines read code.md

# Folders

Folder rules contains generic rules for ai agent (you)

Folder prj contains project requirements and info

prj/goal.md is a general goal of this project

Folder prj/architecture contains project analysis - you agent keep there decisions (libraries, code architecture) and choices - at your discretions, keep it short and readable for humans

Folder work contains work data (done, or to be done)

work/initial contains initial goals, state of the project - read it before, assume it is done (check with code if in doubt)

work/requirements and work/tech-debt folders contain pieces of work to be done

for each requirement or tech-debt there is a folder  with some name

in folder there is a file req_XYZ.md or td_XYZ.md which contains what has to be done

there can be a file plan.md (maintained by ai agent - you) which containt implementation plan for the feature (or tech debt) (keep it short and clear)

if the feature is done there is a report.md file which contains final report (was feature done fully, what problems there were) (keep it short and clear)

**IMPORTANT:** plan.md is a REQUIRED step for every feature. Always create plan.md before starting implementation. Never skip it.

**IMPORTANT:** Never modify an existing requirement file (req_*.md) that already has a report.md — that requirement is considered done. If new related work is needed, create a new requirement folder and file instead.


# Work loop

Ensure you understand code.md

Choose one of the features or tech-debts that are not done yet (no report.md)

If it is feature:

- create plan.md with an implementation plan
- start implementing in steps
- try to keep code compiling after each step (if reasonable)
- after the feature is implemented ensure there are tests for the feature, code must compile, tests must pass

If tech-debt is to be implemented:

- Just do it and write report
- Always ensure that code compiles and tests are passing
- If you see that tech-debt is a generic rule:
  - add section with rule to code.md, check if it is possible to add linter or other tool to build to prevent similar tech-debt in the future



