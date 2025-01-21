import type { PageLoad } from './$types'; // Correct import for the load function type

export type Subject = {
	id: number;
	name: string;
};

// The correct type for the load function
export const load: PageLoad = async () => {
	const response = await fetch('http://0.0.0.0:8080');
	if (!response.ok) {
		throw new Error('Failed to fetch subjects');
	}
	const subjects: Subject[] = await response.json();

	return {
		subjects
	};
};
