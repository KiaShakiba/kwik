pub fn min<T: PartialOrd + Copy>(values: &[&T]) -> T {
	if values.len() == 0 {
		panic!("Cannot find min value");
	}

	let mut min_value: T = *values[0];

	for value in values {
		if **value < min_value {
			min_value = **value;
		}
	}

	min_value
}

pub fn max<T: PartialOrd + Copy>(values: &[&T]) -> T {
	if values.len() == 0 {
		panic!("Cannot find max value");
	}

	let mut max_value: T = *values[0];

	for value in values {
		if **value > max_value {
			max_value = **value;
		}
	}

	max_value
}
