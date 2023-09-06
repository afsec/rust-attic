fn main() -> anyhow::Result<()> {
    Ok(())
}

mod model {
    use std::{ops::Deref, str::FromStr};

    #[derive(Debug, Default)]
    pub(crate) struct Body(String);

    impl FromStr for Body {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Self(s.into()))
        }
    }

    #[derive(Debug)]
    pub(crate) struct Request {
        body: Body,
        created_at: String, // timestamp
    }

    impl Request {
        pub(crate) fn body(&self) -> &Body {
            &self.body
        }

        pub(crate) fn created_at(&self) -> &String {
            &self.created_at
        }
    }

    pub(crate) struct Request1<'a> {
        body: &'a Body,
        created_at: &'a String, // timestamp
    }

    impl<'a> From<&'a Request> for Request1<'a> {
        fn from(req: &'a Request) -> Self {
            Self {
                body: req.body(),
                created_at: req.created_at(),
            }
        }
    }

    #[derive(Debug)]
    pub(crate) struct Request2<'body, 'created_at> {
        body: &'body Body,
        created_at: &'created_at String, // timestamp
    }
    impl<'body, 'created_at> From<(&'body Body, &'created_at String)> for Request2<'body, 'created_at> {
        fn from((body, created_at): (&'body Body, &'created_at String)) -> Self {
            Self { body, created_at }
        }
    }

    impl<'fields> From<&'fields Request> for Request2<'fields, 'fields> {
        fn from(req: &'fields Request) -> Self {
            let body = req.body();
            let created_at = req.created_at();
            Self { body, created_at }
        }
    }
}
