import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = ({ url }) => {
    //   console.log(`help: ${url}`);
    //   console.log(`help: ${route.id}`);
    return {
        url: url.pathname,
    };
};
