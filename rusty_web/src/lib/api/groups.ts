import { fetchPost } from '$lib/utils/api';
import type { Group } from '$lib/domain/group';

const getGroupsQuery = (page: number, name: string) => {
	const filter = `filter: { name: { contains: "${name}" } }, `;
	const options = `options: { pageNumber: ${page}, pageSize: 30, sortMode: ASCENDING, sortField: "name" }`;
	return `query {
		projectGroups {
			get(${filter}${options}){
				total
				page
				pageSize
				entries {
					id
					name
				}
			}
		}
	}`;
};

export const fetchGroups = async (auth: string, page: number, groupName: string) => {
	try {
		const response = await fetchPost(
			auth,
			JSON.stringify({ query: getGroupsQuery(page, groupName) })
		);

		if (!response.ok) {
			return {
				errors: ['Fetch groups failed']
			};
		} else {
			const { data, errors } = await response.json();
			if (errors && errors.length > 0) {
				return {
					errors: errors.map((error: { message: string }) => error.message)
				};
			} else if (data) {
				const paged = data?.projectGroups?.get;
				const groups: Group[] = paged?.entries ?? [];
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					active: { id: '', name: 'Default' },
					entries: groups
				};
			}
		}
	} catch (error) {
		return {
			errors: ['Fetch groups failed']
		};
	}
};
