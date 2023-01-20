<template>
	<div class="card">
		<div class="card-header">
			<a class="card-title" :href="'/post/' + post.post_id">{{ post.title }}</a>
			<p class="card-date">{{ formatDate(post.date_time) }}</p>
		</div>
		<div class="divider"/>
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

	import dayjs from "dayjs";


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
		methods: {
			formatDate(date_epoch: number) {
				return dayjs.unix(date_epoch).format("MMM, D YYYY")
			}
		}
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
			color: var(--color-text);

			text-decoration: none;
			font-size: 1.55rem;
			font-weight: 600;
		}

		.card-date {
			margin: 0;
			margin-left: 0.5rem;

			font-size: 0.9rem;
			font-style: italic;
			text-decoration: underline;
		}
	}
}

</style>
