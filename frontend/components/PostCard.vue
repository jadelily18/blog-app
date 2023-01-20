<template>
	<div class="card">
		<div class="card-header">
			<h1 class="card-title">{{ post.title }}</h1>
			<p class="card-date">{{ post.date_time }}</p>
		</div>
		<div class="card-content">
			{{ post.description }}
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
		props: {
			post: Object
		},
		data: () => ({
			author: {}
		}),
		async fetch() {
			this.author = await fetch(
				"http://localhost:8081/api/user/" + this.post.author_id
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
