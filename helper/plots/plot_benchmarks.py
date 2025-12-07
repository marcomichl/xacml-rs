import re
import matplotlib.pyplot as plt

benchmark_output_m3 = """
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

benchmark_output_rpi= """
test bench_multiple_rule_policy_0                 ... bench:       2,521.40 ns/iter (+/- 102.06)
test bench_multiple_rule_policy_10                ... bench:      26,273.39 ns/iter (+/- 240.75)
test bench_multiple_rule_policy_100               ... bench:     617,646.10 ns/iter (+/- 3,607.65)
test bench_multiple_rule_policy_15                ... bench:      42,509.99 ns/iter (+/- 328.71)
test bench_multiple_rule_policy_20                ... bench:      59,330.01 ns/iter (+/- 826.62)
test bench_multiple_rule_policy_25                ... bench:      79,355.74 ns/iter (+/- 691.31)
test bench_multiple_rule_policy_30                ... bench:     102,254.61 ns/iter (+/- 731.30)
test bench_multiple_rule_policy_35                ... bench:     125,174.78 ns/iter (+/- 2,205.93)
test bench_multiple_rule_policy_40                ... bench:     151,295.64 ns/iter (+/- 1,064.97)
test bench_multiple_rule_policy_45                ... bench:     176,230.80 ns/iter (+/- 1,226.92)
test bench_multiple_rule_policy_5                 ... bench:      13,467.07 ns/iter (+/- 117.99)
test bench_multiple_rule_policy_50                ... bench:     203,870.80 ns/iter (+/- 1,465.44)
test bench_multiple_rule_policy_55                ... bench:     236,562.60 ns/iter (+/- 1,218.18)
test bench_multiple_rule_policy_60                ... bench:     265,032.70 ns/iter (+/- 5,130.64)
test bench_multiple_rule_policy_65                ... bench:     309,085.10 ns/iter (+/- 1,483.44)
test bench_multiple_rule_policy_70                ... bench:     351,035.60 ns/iter (+/- 3,125.19)
test bench_multiple_rule_policy_75                ... bench:     390,027.90 ns/iter (+/- 1,765.83)
test bench_multiple_rule_policy_80                ... bench:     440,803.30 ns/iter (+/- 3,161.82)
test bench_multiple_rule_policy_85                ... bench:     477,664.00 ns/iter (+/- 3,441.07)
test bench_multiple_rule_policy_90                ... bench:     512,236.00 ns/iter (+/- 3,886.87)
test bench_multiple_rule_policy_95                ... bench:     561,317.10 ns/iter (+/- 3,188.57)
"""

benchmark_output_ecu = """
test bench_multiple_rule_policy_0                 ... bench:      34,802.77 ns/iter (+/- 12,227.39)
test bench_multiple_rule_policy_10                ... bench:     368,657.50 ns/iter (+/- 180,104.69)
test bench_multiple_rule_policy_100               ... bench:   8,153,525.00 ns/iter (+/- 2,271,389.00)
test bench_multiple_rule_policy_15                ... bench:     578,093.12 ns/iter (+/- 204,438.94)
test bench_multiple_rule_policy_20                ... bench:     807,498.75 ns/iter (+/- 394,286.12)
test bench_multiple_rule_policy_25                ... bench:   1,076,722.50 ns/iter (+/- 380,978.88)
test bench_multiple_rule_policy_30                ... bench:   1,335,611.25 ns/iter (+/- 414,092.38)
test bench_multiple_rule_policy_35                ... bench:   1,523,345.00 ns/iter (+/- 437,268.00)
test bench_multiple_rule_policy_40                ... bench:   1,805,472.50 ns/iter (+/- 386,218.25)
test bench_multiple_rule_policy_45                ... bench:   2,127,012.50 ns/iter (+/- 343,768.75)
test bench_multiple_rule_policy_5                 ... bench:     179,005.00 ns/iter (+/- 53,987.78)
test bench_multiple_rule_policy_50                ... bench:   2,512,762.50 ns/iter (+/- 621,254.25)
test bench_multiple_rule_policy_55                ... bench:   2,910,317.50 ns/iter (+/- 459,006.25)
test bench_multiple_rule_policy_60                ... bench:   3,293,920.00 ns/iter (+/- 963,716.00)
test bench_multiple_rule_policy_65                ... bench:   3,748,755.00 ns/iter (+/- 1,478,962.50)
test bench_multiple_rule_policy_70                ... bench:   4,241,760.00 ns/iter (+/- 779,316.50)
test bench_multiple_rule_policy_75                ... bench:   4,631,515.00 ns/iter (+/- 723,909.00)
test bench_multiple_rule_policy_80                ... bench:   5,209,880.00 ns/iter (+/- 1,168,456.00)
test bench_multiple_rule_policy_85                ... bench:   5,698,920.00 ns/iter (+/- 656,556.00)
test bench_multiple_rule_policy_90                ... bench:   6,199,305.00 ns/iter (+/- 613,943.50)
test bench_multiple_rule_policy_95                ... bench:   6,849,835.00 ns/iter (+/- 829,190.00)
"""

pattern = re.compile(
    r"bench_multiple_rule_policy_(\d+)\s+.*?bench:\s+([\d,]+\.\d+)\s+ns/iter"
)

data_m3 = []
for match in pattern.finditer(benchmark_output_m3):
    rule_count = int(match.group(1))
    times_us = float(match.group(2).replace(",", ""))/1000
    data_m3.append((rule_count, times_us))

data_rpi = []
for match in pattern.finditer(benchmark_output_rpi):
    rule_count = int(match.group(1))
    times_us = float(match.group(2).replace(",", ""))/1000
    data_rpi.append((rule_count, times_us))

data_ecu = []
for match in pattern.finditer(benchmark_output_ecu):
    rule_count = int(match.group(1))
    times_us = float(match.group(2).replace(",", ""))/1000
    data_ecu.append((rule_count, times_us))

data_m3.sort(key=lambda x: x[0])
data_rpi.sort(key=lambda x: x[0])
data_ecu.sort(key=lambda x: x[0])
rule_counts_m3, times_us_m3 = zip(*data_m3)
rule_counts_rpi, times_us_rpi = zip(*data_rpi)
rule_counts_ecu, times_us_ecu = zip(*data_ecu)

plt.figure(figsize=(5, 3))
plt.plot(rule_counts_m3, times_us_m3, 'bo-')
plt.plot(rule_counts_rpi, times_us_rpi, 'r+-')
plt.plot(rule_counts_ecu, times_us_ecu, 'gs-')
plt.xlabel("Number of Rules")
plt.ylabel("Time per Evaluation (µs)")
plt.grid(True)
plt.tight_layout()
plt.savefig("benchmark_plot_ecu.svg", format="svg")  
plt.show()  