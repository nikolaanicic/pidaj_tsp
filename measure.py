import subprocess


command = ["./target/debug/projekat_2025"]


def parse_distance_from_result(result:str):
	return int(result.split()[0])

def main():
	try:
		avg:int = 0
		runs:int = 100

		for i in range(runs):
			avg += parse_distance_from_result(subprocess.run(command,capture_output=True,text=True,check=True).stdout)
		
		print(f"average distance on {runs} runs is: {avg/runs}km")

	except Exception as ex:
		print(f"Failed to run the command: {ex}")




if __name__ == "__main__":
	main()
