CC = clang++
OUT = start
EXEC: run

all: ${EXEC}
 
build :
	${CC} $(filter-out $@,$(MAKECMDGOALS)) -o ${OUT}
	# g++ $(filter-out $@,$(MAKECMDGOALS)) -fdiagnostics-color

run:
	./${OUT}

clean:
	rm -rf ${OUT}
