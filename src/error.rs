error_chain! {
    types { }

    links { }

    foreign_links { }

    errors {
        PoisonError(description: String, display: String) {
            description(description)
            display(
                "{}",
                display,
            )
        }

        StaticIsAlreadySet {
            description("Error setting static. The static was already set.")
            display(
                "{}",
                "Error setting static. The static was already set."
            )
        }

        StaticWasNeverSet {
            description("Error getting static. The static was never set.")
            display(
                "{}",
                "Error getting static. The static was never set.",
            )
        }
    }
}
