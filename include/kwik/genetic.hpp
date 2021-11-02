#ifndef _GENETIC_HPP_
#define _GENETIC_HPP_

#include <vector>
#include <functional>
#include <algorithm>
#include <cstdlib>

namespace kwik {
	template <typename T>
	class genetic;
};

template <typename T>
class kwik::genetic {
private:
	typedef std::vector<T> values_t;
	typedef std::function<int64_t(const values_t &)> measurer_t;
	typedef std::function<void(T &)> mutator_t;
	typedef std::function<bool(const values_t &)> checker_t;

	struct individual_t {
		private:
			const genetic *algo;

		public:
			values_t values;

			individual_t(const genetic *algo, values_t values) :
				algo(algo), values(values) {}

			bool operator<(const individual_t &rhs) const {
				return this->fitness() < rhs.fitness();
			}

			int64_t fitness() const {
				return std::abs(this->algo->measure(this->values) - this->algo->target);
			}

			individual_t mate(const individual_t &parent) const {
				values_t child_values;

				do {
					for (size_t i = 0; i < this->values.size(); i++) {
						double random_value = kwik::utils::random();

						if (random_value < (1.0 - genetic::MUTATION_PROBABILITY) / 2) {
							child_values.push_back(this->values[i]);
						} else if (random_value < 1.0 - genetic::MUTATION_PROBABILITY) {
							child_values.push_back(parent.values[i]);
						} else {
							T value = this->values[i];
							this->algo->mutate(value);
							child_values.push_back(value);
						}
					}
				} while (!this->algo->check(child_values));

				return individual_t(this->algo, child_values);
			}
	};

	static const uint32_t POPULATION_SIZE = 100;
	static const uint32_t CONVERGENCE_SIZE = 1000;
	static constexpr const double MUTATION_PROBABILITY = 0.1;
	static constexpr const double ELITE_RATIO = 0.1;
	static constexpr const double MATING_RATIO = 0.5;

	std::vector<individual_t> population;
	int64_t target;
	measurer_t measure;
	mutator_t mutate;
	checker_t check;

	uint64_t generation_count = 0;
	uint64_t convergence_count = 0;
	uint64_t last_fitness = 0;

public:
	genetic(
		values_t initial_values,
		int64_t target,
		measurer_t measure,
		mutator_t mutate,
		checker_t check = [](const values_t &) { return true; }
	) : target(target), measure(measure), mutate(mutate), check(check) {
		if (!this->check(initial_values)) {
			throw std::invalid_argument("Initial values do not pass check");
		}

		for (uint32_t i = 0; i < genetic::POPULATION_SIZE; i++) {
			this->population.push_back(individual_t(this, initial_values));
		}
	}

	uint64_t generations() const {
		return this->generation_count;
	}

	values_t run() {
		int64_t last_fitness = this->iterate();
		uint64_t convergence_count = 0;

		while (last_fitness != 0 && convergence_count < genetic::CONVERGENCE_SIZE) {
			int64_t fitness = this->iterate();

			if (fitness != last_fitness) {
				last_fitness = fitness;
				convergence_count = 0;
			} else {
				convergence_count++;
			}
		}

		return this->population[0].values;
	}

private:
	int64_t iterate() {
		this->generation_count++;

		std::vector<individual_t> new_generation (
			this->population.begin(),
			this->population.begin() + genetic::POPULATION_SIZE * genetic::ELITE_RATIO
		);

		for (uint32_t i = 0; i < genetic::POPULATION_SIZE - genetic::POPULATION_SIZE * genetic::ELITE_RATIO; i++) {
			size_t index1 = kwik::utils::random<size_t>(0, genetic::POPULATION_SIZE * genetic::MATING_RATIO);
			size_t index2 = kwik::utils::random<size_t>(0, genetic::POPULATION_SIZE * genetic::MATING_RATIO);

			while (index1 == index2) {
				index2 = kwik::utils::random<size_t>(0, genetic::POPULATION_SIZE * genetic::MATING_RATIO);
			}

			individual_t parent1 = this->population[index1];
			individual_t parent2 = this->population[index2];
			individual_t child = parent1.mate(parent2);

			new_generation.push_back(child);
		}

		std::sort(new_generation.begin(), new_generation.end());

		this->population = new_generation;

		return this->population[0].fitness();
	}
};

#endif
