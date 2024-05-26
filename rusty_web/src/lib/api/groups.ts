import { fetchPost } from '$lib/utils/api';
import type { Group } from '$lib/domain/group';

const getGroupsQuery = (page: number) => `query {
	projectGroups {
		get(options: { pageNumber: ${page}, pageSize: 50 }){
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

const defaultGroup = { id: '', name: 'Default' };

export const fetchGroups = async (auth: string, page: number) => {
	try {
		const response = await fetchPost(auth, JSON.stringify({ query: getGroupsQuery(page) }));

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
				groups.unshift(defaultGroup);
				return {
					total: paged?.total ?? 0,
					page: paged?.page ?? 1,
					pageSize: paged?.pageSize ?? 20,
					active: defaultGroup,
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
