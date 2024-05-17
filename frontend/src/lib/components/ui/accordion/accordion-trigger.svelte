<script>
	import { Accordion as AccordionPrimitive } from "bits-ui";
	import ChevronDown from "svelte-radix/ChevronDown.svelte";
	import { cn } from "/utils.js";
	let className = undefined;
	export let level = 3;
	export { className as class };
</script>

<AccordionPrimitive.Header {level} class="flex">
	<AccordionPrimitive.Trigger
		class={cn(
			"flex flex-1 items-center justify-between py-4 text-sm font-medium transition-all hover:underline [&[data-state=open]>svg]:rotate-180",
			className
		)}
		{...$$restProps}
		on:click
	>
		<slot />
		<ChevronDown
			class="h-4 w-4 shrink-0 text-muted-foreground transition-transform duration-200"
		/>
	</AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
