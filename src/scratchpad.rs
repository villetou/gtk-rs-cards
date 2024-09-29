        // A mutable integer
        let number = Rc::new(Cell::new(0));

        // Add buttons to `gtk_box`
        let gtk_box = gtk::Box::builder()
            .orientation(Orientation::Vertical)
            .build();
        gtk_box.append(&button_increase);
        gtk_box.append(&button_decrease);
        gtk_box.append(&gameboard);
    
    
    // Connect callbacks
    // When a button is clicked, `number` and label of the other button will be changed
    button_increase.connect_clicked(clone!(
        #[weak]
        number,
        #[weak]
        button_decrease,
        move |_| {
            number.set(number.get() + 1);
            button_decrease.set_label(&number.get().to_string());
        }
    ));
    button_decrease.connect_clicked(clone!(
        #[weak]
        button_increase,
        move |_| {
            number.set(number.get() - 1);
            button_increase.set_label(&number.get().to_string());
        }
    ));
    // Add buttons to `gtk_box`
    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    gtk_box.append(&button_increase);
    gtk_box.append(&button_decrease);