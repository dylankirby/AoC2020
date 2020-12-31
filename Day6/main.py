from collections import Counter

def main1():
	with open("data.txt") as fr:
		data = [l.replace("\n", " ") for l in fr.read().split("\n\n")]
		
		ts = 0
		for d in data:
			c = Counter()
			split = d.split(" ")
			split_as_sets = [set([i for i in string]) for string in split]

			_all = set()
			for s in split_as_sets:
				_all.update(s)

			ts += len(_all)

		print(ts)

def main2():
	with open("data.txt") as fr:
		data = [l.replace("\n", " ") for l in fr.read().split("\n\n")]
		
		ts = 0
		for d in data:
			c = Counter()
			split = d.split(" ")
			split_as_sets = [set([i for i in string]) for string in split]
			
			initial_set = split_as_sets.pop(0)
			for s in split_as_sets:
				initial_set.intersection_update(s)

			c.update(initial_set)

			ts += sum(c.values())

		print(ts)



if __name__ == "__main__":
	print("Part 1:")
	main1()
	print("Part 2: ")
	main2()