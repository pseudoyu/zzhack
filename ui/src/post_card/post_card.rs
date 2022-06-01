use crate::link::Link;
use crate::post_card_header::PostCardHeader;
use router::RootRoutes;
use services::post_service::Post;
use stylist::style;
use urlencoding::encode;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct PostCardProps {
    pub post: Post,
}

#[function_component(PostCard)]
pub fn post_card(props: &PostCardProps) -> Html {
    let style = style!(
        r#"
        width: 250px;
        height: 350px;
        background: var(--base-color);
        box-shadow: 0px 7px 43px 0px var(--card-shadow-color);
        border-radius: 17.33px;
        box-sizing: border-box;
        padding: 18px;
        margin-bottom: 79px;
        cursor: pointer;
        margin-right: 38px;

        .cover {
            width: 236px;
            height: 134px;
            border-radius: 10px;
            margin: 13px -11px;
            margin-bottom: 0;
            background-image: url("${cover}");
            background-repeat: no-repeat;
            background-size: cover;
            background-position: 50% 50%;
        }

        .post-preview__title {
            font-size: 15.25px;
            font-weight: bold;
            color: var(--text-color);
            line-height: 26px;
            margin-top: 17px;
        }

        .post-preview__body {
            width: 100%;
            word-break: break-all;
            text-overflow: ellipsis;
            line-height: 22px;
            display: -webkit-box;
            -webkit-line-clamp: 3;
            -webkit-box-orient: vertical;
            overflow: hidden;
            margin-top: 4px;
            font-size: 14.48px;
            color: var(--sub-text-color);
        }

        .modified-at {
            color: var(--tip-color);
            font-size: 10px;
            line-height: 15px;
            margin-top: 10px;
        }

        @media (max-width: 600px) {
            width: 100%;
            margin-bottom: 33px;

            .cover {
                width: 100%;
                margin: 13px 0;
            }
        }
    "#,
        cover = props.post.metadata.cover.clone()
    )
    .unwrap();
    let post_encoded_title = encode(props.post.metadata.title.as_str());

    html! {
        <div class={style}>
            <Link href={RootRoutes::Post{title: post_encoded_title.to_string()}}>
                <div class="wrapper">
                    <PostCardHeader label={props.post.metadata.tag.clone()} />
                    <div class="cover" />
                    <div class="post-preview">
                        <div class="post-preview__title">
                            {&props.post.metadata.title}
                        </div>
                        <div class="post-preview__body">
                            {&props.post.desc}
                        </div>
                    </div>
                    <div class="modified-at">{&props.post.modified_time}</div>
                </div>
            </Link>
        </div>
    }
}
