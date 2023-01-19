<template>
	<div class="card">
		<div class="card-header">
			<h1 class="card-title">{{ this.$vnode.key.title }}</h1>
			<p class="card-date">{{ this.$vnode.key.date_time }}</p>
		</div>
		<div class="card-content">
			{{ this.$vnode.key.description }}
		</div>
		<div class="card-footer">
			by <span>{{ author.first_name }}</span>
		</div>
	</div>

</template>

<script lang="ts">
	import Vue from "vue"
	export default Vue.extend({
		name: "PostCard",
		data: () => ({
			author: {}
		}),
		async fetch() {
			this.author = await fetch(
				"http://localhost:8081/api/user/" + this.$vnode.key.author_id
			).then(res => res.json())
		},
	})
</script>

<style lang="scss" scoped>
.card {
	display: flex;
	flex-direction: column;

	width: 30rem;
	padding: 0.5rem 0.75rem 1rem;

	border-radius: 0.25rem;
	background-color: #363636;

	.card-header {
		display: flex;
		place-items: center;

		.card-title {
			margin: 0;
		}

		.card-date {
			margin-left: 0.5rem;
		}
	}
}

</style>
