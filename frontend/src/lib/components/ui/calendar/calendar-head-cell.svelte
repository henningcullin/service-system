<script>
	import { Calendar as CalendarPrimitive } from "bits-ui";
	import { cn } from "/utils.js";
	let className = undefined;
	export { className as class };
</script>

<CalendarPrimitive.HeadCell
	class={cn("w-8 rounded-md text-[0.8rem] font-normal text-muted-foreground", className)}
	{...$$restProps}
>
	<slot />
</CalendarPrimitive.HeadCell>
