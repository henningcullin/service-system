<script>
	import { Calendar as CalendarPrimitive } from "bits-ui";
	import { cn } from "/utils.js";
	let className = undefined;
	export { className as class };
</script>

<CalendarPrimitive.GridBody class={cn(className)} {...$$restProps}>
	<slot />
</CalendarPrimitive.GridBody>
