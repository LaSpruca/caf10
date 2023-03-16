import type { PageLoad } from './$types';
 
export const load = (({ params: {game, nicname} }) => {
    return {
      nicname,
      game
    };
})