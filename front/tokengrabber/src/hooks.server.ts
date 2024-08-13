import type { Handle } from '@sveltejs/kit';


export const handle: Handle = async ({ event, resolve }) => {
	console.log("startup");
	const response = await resolve(event);

	return response;
}
