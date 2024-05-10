<script>
    import { Item, Divider, List } from "svelte-fomantic-ui";
    import { Link } from "svelte-navigator";

    export let task;
</script>

<Item>
    <div class="image">
        <img src="https://fomantic-ui.com/images/wireframe/image.png" alt="" />
    </div>
    <div class="content">
        <Link to="/task?id={task.id}" class="header">{task.title}</Link>
        <div class="meta">
            <b>Type:</b><Link to="/task/type?id={task.task_type.id}"
                >{task.task_type.name}</Link
            >
            <b>Status: </b><Link to="/task/status?id={task.status.id}"
                >{task.status.name}</Link
            >
            {#if task.due_at}
                <b>Due at: </b>{task.due_at}
            {/if}
        </div>
        <div class="description">
            <span>{task.description}</span>
            <Divider ui />
            <b>Creator</b>
            <List ui divided>
                <div class="item">
                    <img
                        class="ui avatar image"
                        src="https://fomantic-ui.com/images/avatar2/small/rachel.png"
                        alt=""
                    />
                    <div class="content">
                        <Link to="/user?id={task.creator.id}" class="header"
                            >{`${task.creator.first_name} ${task.creator.last_name}`}</Link
                        >
                        <div class="description">
                            <a href="mailto:{task.creator.email}"
                                >{task.creator.email}</a
                            >
                        </div>
                    </div>
                </div>
            </List>
            {#if task.machine}
                <Divider ui />
                <b>Machine</b>
                <List ui divided>
                    <div class="item">
                        <img
                            class="ui avatar image"
                            src="https://fomantic-ui.com/images/avatar2/small/rachel.png"
                            alt=""
                        />
                        <div class="content">
                            <Link
                                to="/machine?id={task.machine.id}"
                                class="header"
                                >{`${task.machine.make} ${task.machine.name}`}</Link
                            >
                        </div>
                    </div>
                </List>
            {/if}
            {#if task.executors}
                <Divider ui />
                <b>Executors</b>
                <List ui divided>
                    {#each task.executors as executor}
                        <div class="item">
                            <img
                                class="ui avatar image"
                                src="https://fomantic-ui.com/images/avatar2/small/rachel.png"
                                alt=""
                            />
                            <div class="content">
                                <Link to="/user?id={executor.id}" class="header"
                                    >{`${executor.first_name} ${executor.last_name}`}</Link
                                >
                                <div class="description">
                                    <a href="mailto:{executor.email}"
                                        >{executor.email}</a
                                    >
                                </div>
                            </div>
                        </div>
                    {/each}
                </List>
            {/if}
        </div>
        <div class="extra">
            <b>Created:</b>{new Date(task.created).toLocaleString()}
            <b>Edited:</b>{new Date(task.edited).toLocaleString()}
        </div>
    </div>
</Item>
