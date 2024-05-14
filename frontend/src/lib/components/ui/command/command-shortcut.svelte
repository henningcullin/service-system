<script>
	import { cn } from "/utils.js";
	let className = undefined;
	export { className as class };
</script>

<span
	class={cn("ml-auto text-xs tracking-widest text-muted-foreground", className)}
	{...$$restProps}
>
	<slot />
</span>
