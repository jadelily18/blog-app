<script lang="ts" setup>
import { Clock } from "lucide-vue-next"

const props = defineProps({
  post: Object
})

const { data: author } = await useFetch(
    "http://localhost:8081/api/user/" + props.post?.author_id
)

</script>

<template>
	<div class="card">
		<div class="card-header">
			<a class="card-title" :href="'/post/' + post.post_id">{{ post.title }}</a>
			<div class="card-date">
        <Clock size="16" />
				<p>{{ formatDate(post.date_time) }}</p>
			</div>
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
import dayjs from "dayjs";

function formatDate(date_epoch: number) {
  return dayjs.unix(date_epoch).format("MMM, D YYYY h:mma")
}

</script>

<style lang="scss" scoped>
.card {
	display: flex;
	flex-direction: column;

	width: 30rem;
	padding: 0.75rem 1rem 1.15rem;

	border-radius: 0.25rem;
	background-color: #363636;

	.card-header {
		display: flex;
		justify-content: space-between;
		place-items: center;

		.card-title {
			margin: 0;
			color: var(--color-text);

			text-decoration: none;
			font-size: 1.55rem;
			font-weight: 600;
		}

		.card-date {
			display: inline-flex;
			color: var(--color-text-muted);

      * {
        margin: 0;
      }

      p {
        font-size: 0.9rem;
        font-style: italic;
      }

      svg {
        margin-right: 0.25rem;
      }
		}
	}
}

</style>
