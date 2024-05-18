import tippy, { type Placement, type Props, type SingleTarget } from 'tippy.js';

export interface TooltipOpts {
	content: string;
	placement: Placement;
}

export function tooltip(node: SingleTarget, props: TooltipOpts) {
	let tip = tippy(node, props);
	return {
		update: (newParams: Pick<Props, 'content' | 'placement'>) => {
			tip.setProps(newParams);
		},
		destroy: () => {
			tip.destroy();
		}
	};
}
