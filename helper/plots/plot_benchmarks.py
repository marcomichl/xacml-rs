import re
import matplotlib.pyplot as plt

benchmark_output = """
test bench_multiple_rule_policy_0   ... bench:         542.92 ns/iter (+/- 42.01)
test bench_multiple_rule_policy_10  ... bench:       5,755.89 ns/iter (+/- 331.20)
test bench_multiple_rule_policy_100 ... bench:      89,834.58 ns/iter (+/- 5,577.65)
test bench_multiple_rule_policy_15  ... bench:       8,920.31 ns/iter (+/- 339.86)
test bench_multiple_rule_policy_20  ... bench:      11,841.39 ns/iter (+/- 217.15)
test bench_multiple_rule_policy_25  ... bench:      15,403.65 ns/iter (+/- 193.22)
test bench_multiple_rule_policy_30  ... bench:      19,293.59 ns/iter (+/- 1,017.95)
test bench_multiple_rule_policy_35  ... bench:      22,882.19 ns/iter (+/- 126.65)
test bench_multiple_rule_policy_40  ... bench:      25,360.22 ns/iter (+/- 776.85)
test bench_multiple_rule_policy_45  ... bench:      30,234.13 ns/iter (+/- 2,784.79)
test bench_multiple_rule_policy_5   ... bench:       2,929.28 ns/iter (+/- 82.51)
test bench_multiple_rule_policy_50  ... bench:      34,139.96 ns/iter (+/- 826.58)
test bench_multiple_rule_policy_55  ... bench:      39,524.07 ns/iter (+/- 2,112.45)
test bench_multiple_rule_policy_60  ... bench:      42,961.67 ns/iter (+/- 838.10)
test bench_multiple_rule_policy_65  ... bench:      47,419.73 ns/iter (+/- 421.23)
test bench_multiple_rule_policy_70  ... bench:      54,503.12 ns/iter (+/- 244.99)
test bench_multiple_rule_policy_75  ... bench:      58,444.17 ns/iter (+/- 2,900.22)
test bench_multiple_rule_policy_80  ... bench:      63,709.52 ns/iter (+/- 244.52)
test bench_multiple_rule_policy_85  ... bench:      69,059.02 ns/iter (+/- 4,125.66)
test bench_multiple_rule_policy_90  ... bench:      73,934.72 ns/iter (+/- 298.16)
test bench_multiple_rule_policy_95  ... bench:      81,065.91 ns/iter (+/- 5,047.85)
"""

pattern = re.compile(
    r"bench_multiple_rule_policy_(\d+)\s+.*?bench:\s+([\d,]+\.\d+)\s+ns/iter"
)

data = []
for match in pattern.finditer(benchmark_output):
    rule_count = int(match.group(1))
    times_us = float(match.group(2).replace(",", ""))/1000
    data.append((rule_count, times_us))

data.sort(key=lambda x: x[0])
rule_counts, times_us = zip(*data)

plt.figure(figsize=(5, 3))
plt.plot(rule_counts, times_us)
plt.xlabel("Number of Rules")
plt.ylabel("Time per Evaluation (µs)")
plt.grid(True)
plt.tight_layout()
plt.savefig("benchmark_plot.svg", format="svg")  
plt.show()  